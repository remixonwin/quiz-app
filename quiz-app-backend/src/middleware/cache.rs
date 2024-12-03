use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use futures::future::{ok, Ready};
use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
    time::Duration,
    num::NonZeroUsize,
};
use tokio::sync::RwLock;
use lru::LruCache;

const CACHE_SIZE: usize = 100;
const CACHE_DURATION: Duration = Duration::from_secs(300); // 5 minutes

#[derive(Clone)]
pub struct CacheMiddleware {
    cache: Arc<RwLock<LruCache<String, (Vec<u8>, std::time::Instant)>>>,
}

impl CacheMiddleware {
    pub fn new() -> Self {
        CacheMiddleware {
            cache: Arc::new(RwLock::new(LruCache::new(
                NonZeroUsize::new(CACHE_SIZE).unwrap()
            ))),
        }
    }
}

impl<S> Transform<S, ServiceRequest> for CacheMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static + Clone,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Transform = CacheMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CacheMiddlewareService {
            service,
            cache: self.cache.clone(),
        })
    }
}

pub struct CacheMiddlewareService<S> {
    service: S,
    cache: Arc<RwLock<LruCache<String, (Vec<u8>, std::time::Instant)>>>,
}

impl<S> Service<ServiceRequest> for CacheMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static + Clone,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<ServiceResponse, Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Only cache GET requests
        if req.method() != "GET" {
            let fut = self.service.call(req);
            return Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            });
        }

        let cache = self.cache.clone();
        let cache_key = format!("{}:{}", req.method(), req.path());
        let service = self.service.clone();

        Box::pin(async move {
            // Try to get from cache
            if let Some((cached_body, timestamp)) = cache.write().await.get(&cache_key) {
                if timestamp.elapsed() < CACHE_DURATION {
                    return Ok(ServiceResponse::new(
                        req.into_parts().0,
                        HttpResponse::Ok()
                            .content_type("application/json")
                            .body(cached_body.clone()),
                    ));
                }
            }

            // If not in cache or expired, call service
            let res = service.call(req).await?;
            let (req, mut response) = res.into_parts();

            // Cache the response if it's successful
            if response.status().is_success() {
                if let Ok(body) = response.map_body(|head, body| body).try_into_bytes() {
                    cache.write().await.put(
                        cache_key,
                        (body.to_vec(), std::time::Instant::now()),
                    );
                }
            }

            Ok(ServiceResponse::new(req, response))
        })
    }
}

// Helper function to invalidate cache for specific paths
pub async fn invalidate_cache(
    cache: Arc<RwLock<LruCache<String, (Vec<u8>, std::time::Instant)>>>,
    path: &str,
) {
    let mut cache = cache.write().await;
    cache.pop(&format!("GET:{}", path));
}
