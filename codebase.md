# Project Documentation
Generated at 2024-12-07 14:30:43

## Table of Contents

1. [commands.js](#commands.js)
2. [quiz.rs](#quiz.rs)
3. [password.rs](#password.rs)
4. [tests.rs](#tests.rs)
5. [error.rs](#error.rs)
6. [jwt.rs](#jwt.rs)
7. [mod.rs](#mod.rs)
8. [seeder.rs](#seeder.rs)
9. [utils.rs](#utils.rs)
10. [error.rs](#error.rs)
11. [mod.rs](#mod.rs)
12. [auth.rs](#auth.rs)
13. [test_helpers.rs](#test_helpers.rs)
14. [quiz.rs](#quiz.rs)
15. [quiz_attempt.rs](#quiz_attempt.rs)
16. [answer.rs](#answer.rs)
17. [user.rs](#user.rs)
18. [question.rs](#question.rs)
19. [mod.rs](#mod.rs)
20. [config.rs](#config.rs)
21. [main.rs](#main.rs)
22. [quiz.rs](#quiz.rs)
23. [answer.rs](#answer.rs)
24. [user.rs](#user.rs)
25. [question.rs](#question.rs)
26. [mod.rs](#mod.rs)
27. [seed.rs](#seed.rs)
28. [db.rs](#db.rs)
29. [lib.rs](#lib.rs)
30. [test_helpers.rs](#test_helpers.rs)
31. [integration_test.rs](#integration_test.rs)
32. [handlers_test.rs](#handlers_test.rs)
33. [question_test.rs](#question_test.rs)
34. [test_helpers.rs](#test_helpers.rs)
35. [mod.rs](#mod.rs)
36. [test_utils.rs](#test_utils.rs)
37. [test_utils.rs](#test_utils.rs)
38. [auth_test.rs](#auth_test.rs)
39. [db_test.rs](#db_test.rs)


## commands.js



## quiz.rs



## Quiz



```rs
# [derive (Debug , Serialize , Deserialize)] pub struct Quiz { pub id : String , pub title : String , pub questions : Vec < Question > , }
```

## Question



```rs
# [derive (Debug , Serialize , Deserialize)] pub struct Question { pub id : String , pub text : String , pub options : Vec < String > , pub correct_answer : usize , }
```

## password.rs



## hash_password



```rs
pub fn hash_password (password : & str) -> Result < String , AppError > { hash (password . as_bytes () , DEFAULT_COST) . map_err (| _ | AppError :: InternalServerError ("Failed to hash password" . to_string ())) }
```

## verify_password



```rs
pub fn verify_password (password : & str , hash : & str) -> Result < bool , AppError > { verify (password . as_bytes () , hash) . map_err (| _ | AppError :: InternalServerError ("Failed to verify password" . to_string ())) }
```

## tests.rs



## tests



## test_jwt_token_creation_and_validation



```rs
# [test] fn test_jwt_token_creation_and_validation () { let user_id = Uuid :: new_v4 () ; let role = "user" . to_string () ; let token = generate_token (user_id , & role) . unwrap () ; assert ! (! token . is_empty ()) ; let claims = validate_token (& token) . unwrap () ; assert_eq ! (claims . sub , user_id) ; assert_eq ! (claims . role , role) ; }
```

## test_invalid_token



```rs
# [test] fn test_invalid_token () { let result = validate_token ("invalid.token.here") ; assert ! (result . is_err ()) ; }
```

## test_expired_token



```rs
# [test] fn test_expired_token () { let user_id = Uuid :: new_v4 () ; let role = "user" . to_string () ; let token = generate_token (user_id , & role) . unwrap () ; let claims = validate_token (& token) . unwrap () ; assert ! (claims . exp > 0) ; }
```

## error.rs



## AuthError



```rs
# [derive (Debug , Display)] pub enum AuthError { # [display (fmt = "Invalid token")] InvalidToken , # [display (fmt = "Token expired")] TokenExpired , # [display (fmt = "Invalid credentials")] InvalidCredentials , # [display (fmt = "Hash error")] HashError , # [display (fmt = "Token creation failed")] TokenCreation , # [display (fmt = "User not found")] UserNotFound , # [display (fmt = "Unauthorized")] Unauthorized , }
```

## ErrorResponse



```rs
# [derive (Serialize)] struct ErrorResponse { message : String , }
```

## jwt.rs



## Claims



```rs
# [derive (Debug , Serialize , Deserialize)] pub struct Claims { pub sub : Uuid , pub role : String , pub exp : i64 , pub iat : i64 , }
```

## generate_token



```rs
pub fn generate_token (user_id : Uuid , role : & str) -> Result < String , AuthError > { let claims = Claims :: new (user_id , role) ; encode (& Header :: default () , & claims , & EncodingKey :: from_secret (JWT_SECRET) ,) . map_err (| _ | AuthError :: TokenCreation) }
```

## validate_token



```rs
pub fn validate_token (token : & str) -> Result < Claims , AuthError > { decode :: < Claims > (token , & DecodingKey :: from_secret (JWT_SECRET) , & Validation :: default () ,) . map (| data | data . claims) . map_err (| _ | AuthError :: InvalidToken) }
```

## hash_password



```rs
pub fn hash_password (password : & str) -> Result < String , AuthError > { hash (password . as_bytes () , DEFAULT_COST) . map_err (| _ | AuthError :: TokenCreation) }
```

## verify_password



```rs
pub fn verify_password (password : & str , hash : & str) -> Result < bool , AuthError > { verify (password . as_bytes () , hash) . map_err (| _ | AuthError :: InvalidCredentials) }
```

## mod.rs



## jwt



## password



## error



## tests



## seeder.rs



## seed_database



```rs
pub async fn seed_database (pool : & PgPool) -> Result < () , AppError > { sqlx :: query ! ("DELETE FROM answers") . execute (pool) . await ? ; sqlx :: query ! ("DELETE FROM questions") . execute (pool) . await ? ; sqlx :: query ! ("DELETE FROM quiz_attempts") . execute (pool) . await ? ; sqlx :: query ! ("DELETE FROM quizzes") . execute (pool) . await ? ; sqlx :: query ! ("DELETE FROM users") . execute (pool) . await ? ; let user = sqlx :: query_as ! (User , r#"
        INSERT INTO users (username, email, password_hash, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $4)
        RETURNING id, username, email, password_hash, created_at, updated_at
        "# , "test_user" , "test@example.com" , hash_password ("password123") ?, Utc :: now ()) . fetch_one (pool) . await ? ; let quiz_id = sqlx :: query ! (r#"
        INSERT INTO quizzes (title, description, created_by, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $4)
        RETURNING id
        "# , "Sample Quiz" , Some ("A sample quiz for testing") , user . id , Utc :: now ()) . fetch_one (pool) . await ? . id ; let question_id = sqlx :: query ! (r#"
        INSERT INTO questions (quiz_id, question_text, points, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $4)
        RETURNING id
        "# , quiz_id , "What is the capital of France?" , 10 , Utc :: now ()) . fetch_one (pool) . await ? . id ; sqlx :: query ! (r#"
        INSERT INTO answers (question_id, answer_text, is_correct, created_at, updated_at)
        VALUES 
        ($1, $2, $3, $4, $4),
        ($1, $5, $6, $4, $4),
        ($1, $7, $8, $4, $4),
        ($1, $9, $10, $4, $4)
        "# , question_id , "Paris" , true , Utc :: now () , "London" , false , "Berlin" , false , "Madrid" , false) . execute (pool) . await ? ; Ok (()) }
```

## utils.rs



## hash_password



```rs
pub fn hash_password (password : & str) -> Result < String , AppError > { hash (password , DEFAULT_COST) . map_err (| _ | AppError :: HashError) }
```

## verify_password



```rs
pub fn verify_password (password : & str , hash : & str) -> Result < bool , AppError > { verify (password , hash) . map_err (| _ | AppError :: HashError) }
```

## error.rs



## AppError



```rs
# [derive (Debug , Display)] pub enum AppError { DatabaseError (String) , NotFound (String) , BadRequest (String) , Unauthorized (String) , Forbidden (String) , ValidationError (String) , AuthError (String) , AuthenticationError (String) , InternalServerError (String) , HashError , }
```

## ErrorResponse



```rs
# [derive (Serialize)] struct ErrorResponse { message : String , }
```

## mod.rs



## auth



## auth.rs



## Auth



```rs
pub struct Auth ;
```

## AuthMiddleware



```rs
pub struct AuthMiddleware < S > { service : S , }
```

## test_helpers.rs



## TestContext



```rs
pub struct TestContext { pub pool : PgPool , pub user_id : Option < Uuid > , pub token : Option < String > , }
```

## quiz.rs



## Quiz



```rs
# [derive (Debug , Serialize , Deserialize , FromRow)] pub struct Quiz { pub id : Uuid , pub title : String , pub description : Option < String > , pub created_by : Uuid , pub created_at : DateTime < Utc > , pub updated_at : DateTime < Utc > , pub completed_at : Option < DateTime < Utc > > , }
```

## CreateQuiz



```rs
# [derive (Debug , Serialize , Deserialize)] pub struct CreateQuiz { pub title : String , pub description : Option < String > , pub created_by : Uuid , }
```

## UpdateQuiz



```rs
# [derive (Debug , Serialize , Deserialize)] pub struct UpdateQuiz { pub id : Uuid , pub title : String , pub description : Option < String > , pub completed_at : Option < DateTime < Utc > > , }
```

## quiz_attempt.rs



## QuizAttempt



```rs
# [derive (Debug , Serialize , Deserialize , FromRow)] pub struct QuizAttempt { pub id : Uuid , pub quiz_id : Uuid , pub user_id : Uuid , pub score : Option < BigDecimal > , pub started_at : DateTime < Utc > , pub completed_at : Option < DateTime < Utc > > , pub created_at : DateTime < Utc > , pub updated_at : DateTime < Utc > , }
```

## CreateQuizAttempt



```rs
# [derive (Debug , Serialize , Deserialize)] pub struct CreateQuizAttempt { pub quiz_id : Uuid , pub user_id : Uuid , pub score : Option < BigDecimal > , pub completed_at : Option < DateTime < Utc > > , }
```

## UpdateQuizAttempt



```rs
# [derive (Debug , Serialize , Deserialize)] pub struct UpdateQuizAttempt { pub id : Uuid , pub score : Option < BigDecimal > , pub completed_at : Option < DateTime < Utc > > , }
```

## answer.rs



## Answer



```rs
# [derive (Debug , Serialize , Deserialize , FromRow)] pub struct Answer { pub id : Uuid , pub question_id : Uuid , pub answer_text : String , pub order_num : i32 , pub is_correct : bool , pub created_at : DateTime < Utc > , pub updated_at : DateTime < Utc > , }
```

## CreateAnswer



```rs
# [derive (Debug , Serialize , Deserialize)] pub struct CreateAnswer { pub question_id : Uuid , pub text : String , pub is_correct : bool , pub order_num : i32 , }
```

## UpdateAnswer



```rs
# [derive (Debug , Serialize , Deserialize)] pub struct UpdateAnswer { pub id : Uuid , pub question_id : Uuid , pub answer_text : String , pub order_num : i32 , pub is_correct : bool , }
```

## UserAnswer



```rs
# [derive (Debug , Serialize , Deserialize)] pub struct UserAnswer { pub question_id : Uuid , pub selected_answer_id : Uuid , }
```

## user.rs



## User



```rs
# [derive (Debug , Serialize , Deserialize)] pub struct User { pub id : Uuid , pub username : String , pub email : String , # [serde (skip_serializing)] pub password_hash : String , pub created_at : DateTime < Utc > , pub updated_at : DateTime < Utc > , pub role : String , }
```

## CreateUser



```rs
# [derive (Debug , Serialize , Deserialize , Clone)] pub struct CreateUser { pub username : String , pub email : String , pub password : String , }
```

## LoginCredentials



```rs
# [derive (Debug , Serialize , Deserialize)] pub struct LoginCredentials { pub email : String , pub password : String , }
```

## UpdateUser



```rs
# [derive (Debug , Deserialize)] pub struct UpdateUser { pub id : Uuid , pub username : Option < String > , pub email : Option < String > , pub password : Option < String > , }
```

## question.rs



## Question



```rs
# [derive (Debug , Serialize , Deserialize , sqlx :: FromRow)] pub struct Question { pub id : Uuid , pub quiz_id : Uuid , pub question_text : String , pub order_num : i32 , pub points : i32 , pub created_at : DateTime < Utc > , pub updated_at : DateTime < Utc > , }
```

## CreateQuestion



```rs
# [derive (Debug , Serialize , Deserialize)] pub struct CreateQuestion { pub quiz_id : Uuid , pub text : String , pub order_num : i32 , }
```

## UpdateQuestion



```rs
# [derive (Debug , Serialize , Deserialize)] pub struct UpdateQuestion { pub id : Uuid , pub quiz_id : Uuid , pub question_text : String , pub order_num : i32 , pub points : Option < i32 > , }
```

## mod.rs



## answer



## question



## quiz



## quiz_attempt



## user



## config.rs



## Config



```rs
# [derive (Debug , Clone)] pub struct Config { pub database_url : String , pub jwt_secret : String , }
```

## get_config



```rs
pub fn get_config () -> Result < Config , env :: VarError > { Ok (Config { database_url : get_database_url () , jwt_secret : get_jwt_secret () , }) }
```

## get_database_url



```rs
pub fn get_database_url () -> String { std :: env :: var ("DATABASE_URL") . unwrap_or_else (| _ | { "postgresql://remixonwin:600181@localhost/quiz_app" . to_string () }) }
```

## get_jwt_secret



```rs
pub fn get_jwt_secret () -> String { std :: env :: var ("JWT_SECRET") . unwrap_or_else (| _ | { "your-256-bit-secret" . to_string () }) }
```

## main.rs



## auth



## handlers



## models



## error



## middleware



## config



## health_check



```rs
async fn health_check () -> HttpResponse { HttpResponse :: Ok () . finish () }
```

## configure_app



```rs
pub fn configure_app (cfg : & mut web :: ServiceConfig) { cfg . service (web :: scope ("/api") . service (web :: resource ("/health") . route (web :: get () . to (health_check))) . service (web :: scope ("/auth") . service (register) . service (login)) . service (web :: scope ("/quizzes") . wrap (Auth) . service (create_quiz) . service (get_quiz) . service (update_quiz) . service (delete_quiz) . service (submit_quiz))) ; }
```

## main



```rs
# [actix_web :: main] async fn main () -> std :: io :: Result < () > { dotenv :: dotenv () . ok () ; env_logger :: init () ; let database_url = std :: env :: var ("DATABASE_URL") . expect ("DATABASE_URL must be set") ; let pool = PgPool :: connect (& database_url) . await . expect ("Failed to connect to Postgres") ; println ! ("🚀 Server starting on http://localhost:8080") ; HttpServer :: new (move | | { let cors = Cors :: default () . allowed_origin ("http://localhost:3000") . allowed_methods (vec ! ["GET" , "POST" , "PUT" , "DELETE"]) . allowed_headers (vec ! [header :: AUTHORIZATION , header :: ACCEPT , header :: CONTENT_TYPE]) . supports_credentials () ; App :: new () . wrap (cors) . wrap (Logger :: default ()) . app_data (web :: Data :: new (pool . clone ())) . configure (configure_app) }) . bind ("127.0.0.1:8080") ? . run () . await }
```

## quiz.rs



## create_quiz



```rs
# [post ("")] pub async fn create_quiz (pool : web :: Data < PgPool > , quiz_data : Json < CreateQuiz > , claims : Claims ,) -> Result < HttpResponse , AppError > { let mut form = quiz_data . into_inner () ; form . created_by = claims . sub ; let quiz = Quiz :: create (& pool , form) . await ? ; Ok (HttpResponse :: Created () . json (quiz)) }
```

## get_quiz



```rs
# [get ("/{id}")] pub async fn get_quiz (pool : web :: Data < PgPool > , id : Path < Uuid > ,) -> Result < HttpResponse , AppError > { let quiz = Quiz :: get_by_id (& pool , id . into_inner ()) . await ? . ok_or_else (| | AppError :: NotFound ("Quiz not found" . into ())) ? ; Ok (HttpResponse :: Ok () . json (quiz)) }
```

## update_quiz



```rs
# [put ("/{id}")] pub async fn update_quiz (pool : web :: Data < PgPool > , id : Path < Uuid > , quiz_data : Json < UpdateQuiz > , claims : Claims ,) -> Result < HttpResponse , AppError > { let quiz = Quiz :: get_by_id (& pool , id . into_inner ()) . await ? . ok_or_else (| | AppError :: NotFound ("Quiz not found" . into ())) ? ; if quiz . created_by != claims . sub { return Err (AppError :: Forbidden ("You can only update your own quizzes" . into ())) ; } let mut form = quiz_data . into_inner () ; form . id = quiz . id ; let updated_quiz = Quiz :: update (& pool , form) . await ? ; Ok (HttpResponse :: Ok () . json (updated_quiz)) }
```

## delete_quiz



```rs
# [delete ("/{id}")] pub async fn delete_quiz (pool : web :: Data < PgPool > , id : Path < Uuid > , claims : Claims ,) -> Result < HttpResponse , AppError > { let quiz = Quiz :: get_by_id (& pool , id . into_inner ()) . await ? . ok_or_else (| | AppError :: NotFound ("Quiz not found" . into ())) ? ; if quiz . created_by != claims . sub { return Err (AppError :: Forbidden ("You can only delete your own quizzes" . into ())) ; } Quiz :: delete (& pool , quiz . id) . await ? ; Ok (HttpResponse :: NoContent () . finish ()) }
```

## submit_quiz



```rs
# [get ("/{id}/submit")] pub async fn submit_quiz (pool : web :: Data < PgPool > , id : Path < Uuid > , claims : Claims ,) -> Result < HttpResponse , AppError > { let quiz = Quiz :: get_by_id (& pool , id . into_inner ()) . await ? . ok_or_else (| | AppError :: NotFound ("Quiz not found" . into ())) ? ; if quiz . created_by != claims . sub { return Err (AppError :: Forbidden ("You can only submit your own quizzes" . into ())) ; } let _questions = Question :: get_by_quiz_id (& pool , quiz . id) . await ? ; Ok (HttpResponse :: Ok () . json (quiz)) }
```

## answer.rs



## get_quiz_answers



```rs
# [get ("/quiz/{quiz_id}/answers")] pub async fn get_quiz_answers (pool : web :: Data < PgPool > , quiz_id : web :: Path < Uuid > , _claims : Claims ,) -> Result < HttpResponse , AppError > { let quiz = match Quiz :: get_by_id (& pool , quiz_id . into_inner ()) . await ? { Some (quiz) => quiz , None => return Err (AppError :: NotFound ("Quiz not found" . into ())) , } ; let answers = Answer :: get_by_quiz_id (& pool , quiz . id) . await ? ; Ok (HttpResponse :: Ok () . json (answers)) }
```

## create_answer



```rs
# [post ("/quiz/{quiz_id}/answer")] pub async fn create_answer (pool : web :: Data < PgPool > , quiz_id : web :: Path < Uuid > , claims : Claims , form : web :: Json < CreateAnswer > ,) -> Result < HttpResponse , AppError > { let quiz = match Quiz :: get_by_id (& pool , quiz_id . into_inner ()) . await ? { Some (quiz) => quiz , None => return Err (AppError :: NotFound ("Quiz not found" . into ())) , } ; if quiz . created_by != claims . sub { return Err (AppError :: Unauthorized ("Not authorized to create answers for this quiz" . into ())) ; } let answer = Answer :: create (& pool , form . into_inner ()) . await ? ; Ok (HttpResponse :: Created () . json (answer)) }
```

## update_answer



```rs
# [put ("/quiz/{quiz_id}/answer/{answer_id}")] pub async fn update_answer (pool : web :: Data < PgPool > , quiz_id : web :: Path < Uuid > , answer_id : web :: Path < Uuid > , claims : Claims , form : web :: Json < UpdateAnswer > ,) -> Result < HttpResponse , AppError > { let quiz = match Quiz :: get_by_id (& pool , quiz_id . into_inner ()) . await ? { Some (quiz) => quiz , None => return Err (AppError :: NotFound ("Quiz not found" . into ())) , } ; if quiz . created_by != claims . sub { return Err (AppError :: Unauthorized ("Not authorized to update this answer" . into ())) ; } let answer_id = answer_id . into_inner () ; let answer = match Answer :: get_by_id (& pool , answer_id) . await ? { Some (answer) => answer , None => return Err (AppError :: NotFound ("Answer not found" . into ())) , } ; let question = match Question :: get_by_id (& pool , answer . question_id) . await ? { Some (question) => question , None => return Err (AppError :: NotFound ("Question not found" . into ())) , } ; if question . quiz_id != quiz . id { return Err (AppError :: BadRequest ("Answer does not belong to this quiz" . into ())) ; } let mut form = form . into_inner () ; form . id = answer_id ; let updated_answer = Answer :: update (& pool , form) . await ? ; Ok (HttpResponse :: Ok () . json (updated_answer)) }
```

## delete_answer



```rs
# [delete ("/quiz/{quiz_id}/answer/{answer_id}")] pub async fn delete_answer (pool : web :: Data < PgPool > , quiz_id : web :: Path < Uuid > , answer_id : web :: Path < Uuid > , claims : Claims ,) -> Result < HttpResponse , AppError > { let quiz = match Quiz :: get_by_id (& pool , quiz_id . into_inner ()) . await ? { Some (quiz) => quiz , None => return Err (AppError :: NotFound ("Quiz not found" . into ())) , } ; if quiz . created_by != claims . sub { return Err (AppError :: Unauthorized ("Not authorized to delete this answer" . into ())) ; } let answer_id = answer_id . into_inner () ; let answer = match Answer :: get_by_id (& pool , answer_id) . await ? { Some (answer) => answer , None => return Err (AppError :: NotFound ("Answer not found" . into ())) , } ; let question = match Question :: get_by_id (& pool , answer . question_id) . await ? { Some (question) => question , None => return Err (AppError :: NotFound ("Question not found" . into ())) , } ; if question . quiz_id != quiz . id { return Err (AppError :: BadRequest ("Answer does not belong to this quiz" . into ())) ; } Answer :: delete (& pool , answer_id) . await ? ; Ok (HttpResponse :: NoContent () . finish ()) }
```

## user.rs



## register



```rs
# [post ("/register")] pub async fn register (pool : web :: Data < PgPool > , user_data : Json < CreateUser > ,) -> Result < HttpResponse , AppError > { let user = User :: create (& pool , user_data . into_inner ()) . await ? ; let token = generate_token (user . id , "user") ? ; Ok (HttpResponse :: Created () . json (json ! ({ "user" : user , "token" : token }))) }
```

## login



```rs
# [post ("/login")] pub async fn login (pool : web :: Data < PgPool > , credentials : Json < LoginCredentials > ,) -> Result < HttpResponse , AppError > { let user = User :: verify_login (& pool , & credentials) . await ? . ok_or_else (| | AppError :: Unauthorized ("Invalid email or password" . into ())) ? ; let token = generate_token (user . id , "user") ? ; Ok (HttpResponse :: Ok () . json (json ! ({ "user" : user , "token" : token }))) }
```

## get_profile



```rs
# [get ("/me")] pub async fn get_profile (pool : web :: Data < PgPool > , claims : Claims ,) -> Result < HttpResponse , AppError > { let user = User :: get_by_id (& pool , claims . sub) . await ? . ok_or_else (| | AppError :: NotFound ("User not found" . into ())) ? ; Ok (HttpResponse :: Ok () . json (user)) }
```

## update_profile



```rs
# [put ("/me")] pub async fn update_profile (pool : web :: Data < PgPool > , user_data : Json < UpdateUser > , claims : Claims ,) -> Result < HttpResponse , AppError > { let mut form = user_data . into_inner () ; form . id = claims . sub ; if let Some (ref password) = form . password { form . password = Some (hash_password (password) ?) ; } let user = User :: update (& pool , form) . await ? ; Ok (HttpResponse :: Ok () . json (user)) }
```

## question.rs



## get_questions



```rs
# [get ("/quiz/{quiz_id}/questions")] pub async fn get_questions (pool : web :: Data < PgPool > , _quiz_id : web :: Path < Uuid > , _claims : Claims ,) -> Result < HttpResponse , AppError > { let questions = Question :: get_by_quiz_id (& pool , _quiz_id . into_inner ()) . await ? ; Ok (HttpResponse :: Ok () . json (questions)) }
```

## create_question



```rs
# [post ("/quiz/{quiz_id}/question")] pub async fn create_question (pool : web :: Data < PgPool > , quiz_id : web :: Path < Uuid > , question_data : Json < CreateQuestion > , claims : Claims ,) -> Result < HttpResponse , AppError > { let quiz = Quiz :: get_by_id (& pool , quiz_id . into_inner ()) . await ? . ok_or_else (| | AppError :: NotFound ("Quiz not found" . into ())) ? ; if quiz . created_by != claims . sub { return Err (AppError :: Forbidden ("You can only add questions to your own quizzes" . into ())) ; } let question = Question :: create (& pool , question_data . into_inner ()) . await ? ; Ok (HttpResponse :: Created () . json (question)) }
```

## get_question



```rs
# [get ("/quiz/{quiz_id}/question/{question_id}")] pub async fn get_question (pool : web :: Data < PgPool > , _quiz_id : web :: Path < Uuid > , question_id : web :: Path < Uuid > , _claims : Claims ,) -> Result < HttpResponse , AppError > { let question = Question :: get_by_id (& pool , question_id . into_inner ()) . await ? ; Ok (HttpResponse :: Ok () . json (question)) }
```

## update_question



```rs
# [put ("/quiz/{quiz_id}/question/{question_id}")] pub async fn update_question (pool : web :: Data < PgPool > , _quiz_id : web :: Path < Uuid > , question_id : web :: Path < Uuid > , question_data : Json < UpdateQuestion > , claims : Claims ,) -> Result < HttpResponse , AppError > { let quiz = Quiz :: get_by_id (& pool , _quiz_id . into_inner ()) . await ? . ok_or_else (| | AppError :: NotFound ("Quiz not found" . into ())) ? ; if quiz . created_by != claims . sub { return Err (AppError :: Forbidden ("You can only update questions in your own quizzes" . into ())) ; } let mut form = question_data . into_inner () ; form . id = question_id . into_inner () ; let question = Question :: update (& pool , form) . await ? ; Ok (HttpResponse :: Ok () . json (question)) }
```

## delete_question



```rs
# [delete ("/quiz/{quiz_id}/question/{question_id}")] pub async fn delete_question (pool : web :: Data < PgPool > , _quiz_id : web :: Path < Uuid > , question_id : web :: Path < Uuid > , claims : Claims ,) -> Result < HttpResponse , AppError > { let quiz = Quiz :: get_by_id (& pool , _quiz_id . into_inner ()) . await ? . ok_or_else (| | AppError :: NotFound ("Quiz not found" . into ())) ? ; if quiz . created_by != claims . sub { return Err (AppError :: Forbidden ("You can only delete questions from your own quizzes" . into ())) ; } Question :: delete (& pool , question_id . into_inner ()) . await ? ; Ok (HttpResponse :: NoContent () . finish ()) }
```

## create_answer



```rs
# [post ("/quiz/{quiz_id}/answer")] pub async fn create_answer (pool : web :: Data < PgPool > , quiz_id : web :: Path < Uuid > , claims : Claims , form : web :: Json < CreateAnswer > ,) -> Result < HttpResponse , AppError > { let quiz = Quiz :: get_by_id (& pool , quiz_id . into_inner ()) . await ? . ok_or_else (| | AppError :: NotFound ("Quiz not found" . into ())) ? ; if quiz . created_by != claims . sub { return Err (AppError :: Unauthorized ("Not authorized to create answers for this quiz" . into ())) ; } let answer = Answer :: create (& pool , form . into_inner ()) . await ? ; Ok (HttpResponse :: Created () . json (answer)) }
```

## get_answers



```rs
# [get ("/answers")] pub async fn get_answers (pool : web :: Data < PgPool > , _claims : Claims ,) -> Result < HttpResponse , AppError > { let answers = Answer :: get_all (& pool) . await ? ; Ok (HttpResponse :: Ok () . json (answers)) }
```

## mod.rs



## user



## quiz



## question



## answer



## seed.rs



## main



```rs
# [tokio :: main] async fn main () -> Result < () , Box < dyn std :: error :: Error > > { let config = config :: get_config () ? ; let pool = PgPool :: connect (& config . database_url) . await ? ; println ! ("Seeding database...") ; Ok (()) }
```

## db.rs



## init_db



```rs
pub async fn init_db (pool : & sqlx :: postgres :: PgPool) -> Result < () , sqlx :: Error > { sqlx :: query ! ("DROP TRIGGER IF EXISTS update_quizzes_updated_at ON quizzes CASCADE") . execute (pool) . await ? ; sqlx :: query ! ("DROP FUNCTION IF EXISTS update_updated_at_column CASCADE") . execute (pool) . await ? ; sqlx :: query ! ("DROP TABLE IF EXISTS submitted_answers CASCADE") . execute (pool) . await ? ; sqlx :: query ! ("DROP TABLE IF EXISTS quiz_attempts CASCADE") . execute (pool) . await ? ; sqlx :: query ! ("DROP TABLE IF EXISTS answers CASCADE") . execute (pool) . await ? ; sqlx :: query ! ("DROP TABLE IF EXISTS questions CASCADE") . execute (pool) . await ? ; sqlx :: query ! ("DROP TABLE IF EXISTS quizzes CASCADE") . execute (pool) . await ? ; sqlx :: query ! ("DROP TABLE IF EXISTS users CASCADE") . execute (pool) . await ? ; sqlx :: query ! (r#"
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            username VARCHAR(255) NOT NULL UNIQUE,
            email VARCHAR(255) NOT NULL UNIQUE,
            password_hash VARCHAR(255) NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        "#) . execute (pool) . await ? ; sqlx :: query ! (r#"
        CREATE TABLE IF NOT EXISTS quizzes (
            id SERIAL PRIMARY KEY,
            title VARCHAR(255) NOT NULL,
            description TEXT,
            creator_id INTEGER NOT NULL REFERENCES users(id),
            created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        "#) . execute (pool) . await ? ; sqlx :: query ! (r#"
        CREATE OR REPLACE FUNCTION update_updated_at_column()
        RETURNS TRIGGER AS $$
        BEGIN
            NEW.updated_at = CURRENT_TIMESTAMP;
            RETURN NEW;
        END;
        $$ language 'plpgsql'
        "#) . execute (pool) . await ? ; sqlx :: query ! (r#"
        DROP TRIGGER IF EXISTS update_quizzes_updated_at ON quizzes
        "#) . execute (pool) . await ? ; sqlx :: query ! (r#"
        CREATE TRIGGER update_quizzes_updated_at
            BEFORE UPDATE ON quizzes
            FOR EACH ROW
            EXECUTE FUNCTION update_updated_at_column()
        "#) . execute (pool) . await ? ; sqlx :: query ! (r#"
        CREATE TABLE IF NOT EXISTS questions (
            id SERIAL PRIMARY KEY,
            quiz_id INTEGER NOT NULL REFERENCES quizzes(id) ON DELETE CASCADE,
            text TEXT NOT NULL,
            question_type VARCHAR(50) NOT NULL,
            order_num INTEGER NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
            UNIQUE(quiz_id, order_num)
        )
        "#) . execute (pool) . await ? ; sqlx :: query ! (r#"
        CREATE TABLE IF NOT EXISTS answers (
            id SERIAL PRIMARY KEY,
            question_id INTEGER NOT NULL REFERENCES questions(id) ON DELETE CASCADE,
            text TEXT NOT NULL,
            is_correct BOOLEAN NOT NULL,
            order_num INTEGER NOT NULL,
            UNIQUE(question_id, order_num)
        )
        "#) . execute (pool) . await ? ; sqlx :: query ! (r#"
        CREATE TABLE IF NOT EXISTS quiz_attempts (
            id SERIAL PRIMARY KEY,
            quiz_id INTEGER NOT NULL REFERENCES quizzes(id) ON DELETE CASCADE,
            user_id INTEGER NOT NULL REFERENCES users(id),
            created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
            completed_at TIMESTAMPTZ,
            score INTEGER,
            UNIQUE(quiz_id, user_id, created_at)
        )
        "#) . execute (pool) . await ? ; sqlx :: query ! (r#"
        CREATE TABLE IF NOT EXISTS submitted_answers (
            id SERIAL PRIMARY KEY,
            attempt_id INTEGER NOT NULL REFERENCES quiz_attempts(id) ON DELETE CASCADE,
            question_id INTEGER NOT NULL REFERENCES questions(id),
            answer_id INTEGER NOT NULL REFERENCES answers(id),
            created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
            UNIQUE(attempt_id, question_id)
        )
        "#) . execute (pool) . await ? ; Ok (()) }
```

## create_pool



```rs
pub async fn create_pool (database_url : & str) -> Result < PgPool , sqlx :: Error > { PgPoolOptions :: new () . max_connections (5) . connect (database_url) . await }
```

## tests



## create_test_pool



```rs
async fn create_test_pool () -> Result < PgPool , sqlx :: Error > { let database_url = std :: env :: var ("DATABASE_URL") . unwrap_or_else (| _ | "postgresql://remixonwin:600181@localhost/quiz_app" . to_string ()) ; create_pool (& database_url) . await }
```

## test_database_connection



```rs
# [tokio :: test] async fn test_database_connection () { let pool = create_test_pool () . await . expect ("Failed to create pool") ; let result = sqlx :: query ("SELECT 1") . execute (& pool) . await ; assert ! (result . is_ok ()) ; }
```

## lib.rs



## auth



## config



## db



## error



## handlers



## models



## utils



## test_helpers



## tests



## test_helpers.rs



## TestUser



```rs
# [derive (Debug , Deserialize)] pub struct TestUser { pub id : Uuid , pub username : String , pub token : String , }
```

## TestContext



```rs
pub struct TestContext { pub pool : PgPool , pub app_state : web :: Data < AppState > , pub user_id : Uuid , pub token : String , }
```

## init_test_env



```rs
pub fn init_test_env () { std :: env :: set_var ("JWT_SECRET" , "test_secret_key_for_quiz_app_tests") ; std :: env :: set_var ("DATABASE_URL" , "postgres://postgres:password@localhost:5432/quiz_app_test") ; }
```

## create_test_user



```rs
pub async fn create_test_user (pool : & PgPool) -> (Uuid , String) { let mut attempt = 0 ; let max_attempts = 5 ; while attempt < max_attempts { let username = format ! ("testuser_{}" , attempt) ; let password_hash = bcrypt :: hash ("testpass" , bcrypt :: DEFAULT_COST) . unwrap () ; let user_id = Uuid :: new_v4 () ; match sqlx :: query ! ("INSERT INTO users (id, username, password_hash, role) VALUES ($1, $2, $3, 'user') RETURNING id" , user_id , username , password_hash) . fetch_one (pool) . await { Ok (record) => { let token = generate_token (user_id , "user") . unwrap () ; return (user_id , token) ; } Err (_) => attempt += 1 , } } panic ! ("Failed to create unique test user after multiple attempts") ; }
```

## setup_test_app



```rs
pub async fn setup_test_app (pool : PgPool) -> App < impl Service < ServiceRequest , Response = ServiceResponse , Error = Error > + 'static , > { let app_state = web :: Data :: new (AppState { pool : pool . clone () }) ; App :: new () . app_data (app_state . clone ()) . configure (configure_routes) }
```

## create_test_quiz



```rs
pub async fn create_test_quiz (ctx : & TestContext) -> Quiz { let quiz = CreateQuiz { title : "Test Quiz" . to_string () , description : Some ("A test quiz" . to_string ()) , created_by : ctx . user_id , questions : Vec :: new () , } ; let app = setup_test_app (ctx . pool . clone ()) . await ; let req = test :: TestRequest :: post () . uri ("/api/quizzes") . insert_header (("Authorization" , format ! ("Bearer {}" , ctx . token))) . set_json (& quiz) . to_request () ; let app = test :: init_service (app) . await ; test :: call_and_read_body_json (& app , req) . await }
```

## setup_test_context



```rs
pub async fn setup_test_context (pool : PgPool) -> TestContext { let (user_id , token) = create_test_user (& pool) . await ; let app_state = web :: Data :: new (AppState { pool : pool . clone () }) ; TestContext { pool , app_state , user_id , token , } }
```

## integration_test.rs



## integration_tests



## test_health_check



```rs
# [actix_web :: test] async fn test_health_check () { let ctx = TestContext :: new () . await ; let resp = ctx . make_request (actix_web :: http :: Method :: GET , "/api/health" , Option :: < () > :: None) . await . unwrap () ; assert_eq ! (resp . status () , StatusCode :: OK) ; }
```

## test_user_registration



```rs
# [actix_web :: test] async fn test_user_registration () { let ctx = TestContext :: new () . await ; let user_data = CreateUser { username : "testuser" . to_string () , email : "test@example.com" . to_string () , password : "password123" . to_string () , } ; let resp = ctx . make_request (actix_web :: http :: Method :: POST , "/api/auth/register" , Some (user_data)) . await . unwrap () ; assert_eq ! (resp . status () , StatusCode :: CREATED) ; let _ = ctx . cleanup () . await ; }
```

## test_user_login



```rs
# [actix_web :: test] async fn test_user_login () { let mut ctx = TestContext :: new () . await ; let user = ctx . create_test_user () . await . unwrap () ; let login_data = LoginCredentials { email : user . email , password : "password123" . to_string () , } ; let resp = ctx . make_request (actix_web :: http :: Method :: POST , "/api/auth/login" , Some (login_data)) . await . unwrap () ; assert_eq ! (resp . status () , StatusCode :: OK) ; let _ = ctx . cleanup () . await ; }
```

## tests



## test_user_flow



```rs
# [actix_rt :: test] async fn test_user_flow () { let mut ctx = TestContext :: new () . await ; let email = "test@example.com" . to_string () ; let username = "testuser" . to_string () ; let password = "testpass123" . to_string () ; let resp = ctx . make_request (Method :: POST , "/api/auth/register" , Some (CreateUser { username : username . clone () , email : email . clone () , password : password . clone () , }) ,) . await . expect ("Failed to make register request") ; assert_eq ! (resp . status () . as_u16 () , 201) ; let resp = ctx . make_request (Method :: POST , "/api/auth/login" , Some (LoginCredentials { email , password , }) ,) . await . expect ("Failed to make login request") ; assert_eq ! (resp . status () . as_u16 () , 200) ; ctx . cleanup () . await . unwrap () ; }
```

## handlers_test.rs



## handlers_tests



## test_create_quiz



```rs
# [actix_web :: test] async fn test_create_quiz () { let mut ctx = TestContext :: new () . await ; let _user = ctx . create_test_user () . await . unwrap () ; let quiz_data = CreateQuiz { title : "Test Quiz" . to_string () , description : Some ("A test quiz" . to_string ()) , created_by : ctx . user_id . unwrap () , } ; let resp = ctx . make_request (actix_web :: http :: Method :: POST , "/api/quizzes" , Some (quiz_data)) . await . unwrap () ; assert_eq ! (resp . status () , StatusCode :: CREATED) ; let _ = ctx . cleanup () . await ; }
```

## test_delete_quiz



```rs
# [actix_web :: test] async fn test_delete_quiz () { let mut ctx = TestContext :: new () . await ; let _user = ctx . create_test_user () . await . unwrap () ; let quiz = ctx . create_test_quiz ("Test Quiz" . to_string ()) . await . unwrap () ; let resp = ctx . make_request (actix_web :: http :: Method :: DELETE , & format ! ("/api/quizzes/{}" , quiz . id) , Option :: < () > :: None) . await . unwrap () ; assert_eq ! (resp . status () , StatusCode :: NO_CONTENT) ; let _ = ctx . cleanup () . await ; }
```

## question_test.rs



## question_tests



## test_create_question



```rs
# [actix_web :: test] async fn test_create_question () { let mut ctx = TestContext :: new () . await ; let _user = ctx . create_test_user () . await . unwrap () ; let quiz = ctx . create_test_quiz ("Test Quiz" . to_string ()) . await . unwrap () ; let question_data = CreateQuestion { quiz_id : quiz . id , text : "What is the capital of France?" . to_string () , order_num : 1 , } ; let resp = ctx . make_request (actix_web :: http :: Method :: POST , & format ! ("/api/quizzes/{}/questions" , quiz . id) , Some (question_data)) . await . unwrap () ; assert_eq ! (resp . status () , StatusCode :: CREATED) ; let _ = ctx . cleanup () . await ; }
```

## test_helpers.rs



## test_helpers_work_correctly



```rs
# [actix_web :: test] async fn test_helpers_work_correctly () { let mut ctx = TestContext :: new () . await ; let _user = ctx . create_test_user () . await . unwrap () ; let quiz = ctx . create_test_quiz ("Test Quiz" . to_string ()) . await . unwrap () ; assert_eq ! (quiz . title , "Test Quiz") ; let _ = ctx . cleanup () . await ; }
```

## mod.rs



## test_helpers



## handlers_test



## auth_test



## db_test



## question_test



## integration_test



## test_utils.rs



## setup_test_db



```rs
pub async fn setup_test_db () -> PgPool { INIT . call_once (| | { dotenv :: dotenv () . ok () ; }) ; let database_url = std :: env :: var ("DATABASE_URL") . expect ("DATABASE_URL must be set") ; let pool = PgPool :: connect (& database_url) . await . expect ("Failed to connect to database") ; sqlx :: query ! ("TRUNCATE TABLE users, quizzes, questions, answers CASCADE") . execute (& pool) . await . expect ("Failed to clean test database") ; pool }
```

## create_test_user



```rs
pub async fn create_test_user (pool : & PgPool) -> Result < User , AppError > { let username = format ! ("test_user_{}" , thread_rng () . gen ::< u32 > ()) ; let create_user = CreateUser { username : username . clone () , password : "test_password123" . to_string () , } ; User :: create (pool , create_user) . await }
```

## create_test_quiz



```rs
pub async fn create_test_quiz (pool : & PgPool , user_id : i32) -> Result < Quiz , AppError > { let create_quiz = CreateQuiz { title : format ! ("Test Quiz {}" , thread_rng () . gen ::< u32 > ()) , description : Some ("A test quiz" . to_string ()) , created_by : user_id , } ; Quiz :: create (pool , create_quiz) . await }
```

## create_test_question



```rs
pub async fn create_test_question (pool : & PgPool , quiz_id : i32) -> Result < Question , AppError > { let create_question = CreateQuestion { quiz_id , question_text : format ! ("Test question {}" , thread_rng () . gen ::< u32 > ()) , order_num : Some (1) , } ; Question :: create (pool , create_question) . await }
```

## create_test_answer



```rs
pub async fn create_test_answer (pool : & PgPool , question_id : i32) -> Result < Answer , AppError > { let create_answer = CreateAnswer { text : format ! ("Test answer {}" , thread_rng () . gen ::< u32 > ()) , is_correct : true , } ; sqlx :: query_as ! (Answer , r#"
        INSERT INTO answers (question_id, text, is_correct, order_num)
        VALUES ($1, $2, $3, $4)
        RETURNING id, question_id, text, is_correct, order_num, created_at, updated_at
        "# , question_id , create_answer . text , create_answer . is_correct , 1) . fetch_one (pool) . await . map_err (| e | AppError :: DatabaseError (e)) }
```

## tests



## test_create_test_user



```rs
# [actix_web :: test] async fn test_create_test_user () { let pool = setup_test_db () . await ; let user = create_test_user (& pool) . await . unwrap () ; assert ! (! user . username . is_empty ()) ; assert ! (! user . password_hash . is_empty ()) ; }
```

## test_create_test_quiz



```rs
# [actix_web :: test] async fn test_create_test_quiz () { let pool = setup_test_db () . await ; let user = create_test_user (& pool) . await . unwrap () ; let quiz = create_test_quiz (& pool , user . id) . await . unwrap () ; assert ! (! quiz . title . is_empty ()) ; assert ! (quiz . description . is_some ()) ; }
```

## test_create_test_question



```rs
# [actix_web :: test] async fn test_create_test_question () { let pool = setup_test_db () . await ; let user = create_test_user (& pool) . await . unwrap () ; let quiz = create_test_quiz (& pool , user . id) . await . unwrap () ; let question = create_test_question (& pool , quiz . id) . await . unwrap () ; assert ! (! question . question_text . is_empty ()) ; assert_eq ! (question . quiz_id , quiz . id) ; }
```

## test_create_test_answer



```rs
# [actix_web :: test] async fn test_create_test_answer () { let pool = setup_test_db () . await ; let user = create_test_user (& pool) . await . unwrap () ; let quiz = create_test_quiz (& pool , user . id) . await . unwrap () ; let question = create_test_question (& pool , quiz . id) . await . unwrap () ; let answer = create_test_answer (& pool , question . id) . await . unwrap () ; assert ! (! answer . text . is_empty ()) ; assert_eq ! (answer . question_id , question . id) ; }
```

## test_utils.rs



## setup_test_db



```rs
pub async fn setup_test_db () -> PgPool { let database_url = std :: env :: var ("DATABASE_URL") . unwrap_or_else (| _ | "postgres://postgres:postgres@localhost:5432/quiz_app_test" . to_string ()) ; PgPool :: connect (& database_url) . await . expect ("Failed to connect to test database") }
```

## reset_db



```rs
pub async fn reset_db (pool : & PgPool) -> Result < () , AppError > { sqlx :: query ! ("TRUNCATE TABLE users, quizzes, questions, answers, quiz_attempts CASCADE") . execute (pool) . await ? ; Ok (()) }
```

## create_test_user



```rs
pub fn create_test_user () -> CreateUser { CreateUser { username : format ! ("test_user_{}" , Uuid :: new_v4 ()) , email : format ! ("test_{}@example.com" , Uuid :: new_v4 ()) , password : "password123" . to_string () , } }
```

## create_test_user_in_db



```rs
pub async fn create_test_user_in_db (pool : & PgPool) -> Result < User , AppError > { User :: create (pool , create_test_user ()) . await }
```

## create_test_quiz



```rs
pub fn create_test_quiz (user_id : Uuid) -> CreateQuiz { CreateQuiz { title : format ! ("Test Quiz {}" , Uuid :: new_v4 ()) , description : Some ("A test quiz description" . to_string ()) , created_by : user_id , } }
```

## create_test_quiz_in_db



```rs
pub async fn create_test_quiz_in_db (pool : & PgPool , user_id : Uuid) -> Result < Quiz , AppError > { Quiz :: create (pool , create_test_quiz (user_id)) . await }
```

## create_test_question



```rs
pub fn create_test_question (quiz_id : Uuid) -> CreateQuestion { CreateQuestion { quiz_id , text : "What is the capital of France?" . to_string () , order_num : 1 , } }
```

## create_test_question_in_db



```rs
pub async fn create_test_question_in_db (pool : & PgPool , quiz_id : Uuid) -> Result < Question , AppError > { Question :: create (pool , create_test_question (quiz_id)) . await }
```

## create_test_answer



```rs
pub fn create_test_answer (question_id : Uuid) -> CreateAnswer { CreateAnswer { question_id , text : "Paris" . to_string () , is_correct : true , order_num : 1 , } }
```

## create_test_answer_in_db



```rs
pub async fn create_test_answer_in_db (pool : & PgPool , question_id : Uuid) -> Result < Answer , AppError > { Answer :: create (pool , create_test_answer (question_id)) . await }
```

## cleanup_test_data



```rs
pub async fn cleanup_test_data (pool : & PgPool) -> Result < () , AppError > { sqlx :: query ! ("TRUNCATE TABLE users, quizzes, questions, answers CASCADE") . execute (pool) . await ? ; Ok (()) }
```

## auth_test.rs



## auth_tests



## test_token_generation_and_validation



```rs
# [test] fn test_token_generation_and_validation () { std :: env :: set_var ("JWT_SECRET" , "test_secret_key_for_quiz_app_tests") ; let user_id = Uuid :: new_v4 () ; let token = auth :: generate_token (user_id , "user") . expect ("Failed to generate token") ; let claims = auth :: validate_token (& token) . expect ("Failed to validate token") ; assert_eq ! (claims . sub , user_id) ; assert_eq ! (claims . role , "user") ; }
```

## test_invalid_token



```rs
# [test] fn test_invalid_token () { std :: env :: set_var ("JWT_SECRET" , "test_secret_key_for_quiz_app_tests") ; let result = auth :: validate_token ("invalid.token.here") ; assert ! (result . is_err ()) ; }
```

## test_password_hashing_and_verification



```rs
# [test] fn test_password_hashing_and_verification () { let password = "test_password123" ; let hash = auth :: password :: hash_password (password) . expect ("Failed to hash password") ; let is_valid = auth :: password :: verify_password (password , & hash) . expect ("Failed to verify password") ; assert ! (is_valid) ; }
```

## test_password_verification_with_wrong_password



```rs
# [test] fn test_password_verification_with_wrong_password () { let password = "test_password123" ; let wrong_password = "wrong_password" ; let hash = auth :: password :: hash_password (password) . expect ("Failed to hash password") ; let is_valid = auth :: password :: verify_password (wrong_password , & hash) . expect ("Failed to verify password") ; assert ! (! is_valid) ; }
```

## db_test.rs



## db_tests



## setup



```rs
async fn setup () -> PgPool { dotenv () . ok () ; let database_url = std :: env :: var ("DATABASE_URL") . expect ("DATABASE_URL must be set") ; PgPool :: connect (& database_url) . await . unwrap () }
```

## test_user_creation



```rs
# [tokio :: test] async fn test_user_creation () { let pool = setup () . await ; let new_user = CreateUser { username : format ! ("testuser_{}" , Uuid :: new_v4 ()) , email : format ! ("test_{}@example.com" , Uuid :: new_v4 ()) , password : "password123" . to_string () , } ; let result = User :: create (& pool , new_user . clone ()) . await ; println ! ("User creation result: {:?}" , result) ; assert ! (result . is_ok ()) ; if let Ok (user) = result { let _ = sqlx :: query ! ("DELETE FROM users WHERE id = $1" , user . id) . execute (& pool) . await ; } }
```

## test_quiz_creation



```rs
# [tokio :: test] async fn test_quiz_creation () { let pool = setup () . await ; let user_result = sqlx :: query ! (r#"
            INSERT INTO users (username, password_hash, created_at, role, email)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, username, password_hash, role, created_at, updated_at
            "# , format ! ("quizcreator_{}" , Uuid :: new_v4 ()) , "password123" , Utc :: now () , "user" , format ! ("quizcreator_{}@example.com" , Uuid :: new_v4 ())) . fetch_one (& pool) . await . expect ("Failed to create test user") ; let title = "Test Quiz" . to_string () ; let description = "Test Description" . to_string () ; let created_at = Utc :: now () ; let result = sqlx :: query ! (r#"
            INSERT INTO quizzes (title, description, created_by, created_at)
            VALUES ($1, $2, $3, $4)
            RETURNING id
            "# , title , description , user_result . id , created_at) . fetch_one (& pool) . await ; if let Err (ref e) = result { eprintln ! ("Quiz creation failed: {:?}" , e) ; } assert ! (result . is_ok ()) ; if let Ok (quiz) = result { let _ = sqlx :: query ! ("DELETE FROM quizzes WHERE id = $1" , quiz . id) . execute (& pool) . await ; } let _ = sqlx :: query ! ("DELETE FROM users WHERE id = $1" , user_result . id) . execute (& pool) . await ; }
```
