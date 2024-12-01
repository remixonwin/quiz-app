import { useState, useCallback } from 'react';

interface ValidationRule {
  validate: (value: any) => boolean;
  message: string;
}

interface ValidationRules {
  [key: string]: ValidationRule[];
}

interface Errors {
  [key: string]: string[];
}

export function useFormValidation<T extends object>(
  initialValues: T,
  validationRules: ValidationRules
) {
  const [values, setValues] = useState<T>(initialValues);
  const [errors, setErrors] = useState<Errors>({});
  const [touched, setTouched] = useState<{ [key: string]: boolean }>({});

  const validateField = useCallback(
    (name: string, value: any) => {
      const fieldRules = validationRules[name] || [];
      const fieldErrors = fieldRules
        .filter(rule => !rule.validate(value))
        .map(rule => rule.message);
      
      return fieldErrors;
    },
    [validationRules]
  );

  const handleChange = useCallback(
    (name: keyof T, value: any) => {
      setValues(prev => ({ ...prev, [name]: value }));
      if (touched[name as string]) {
        const fieldErrors = validateField(name as string, value);
        setErrors(prev => ({ ...prev, [name]: fieldErrors }));
      }
    },
    [touched, validateField]
  );

  const handleBlur = useCallback(
    (name: string) => {
      setTouched(prev => ({ ...prev, [name]: true }));
      const fieldErrors = validateField(name, values[name as keyof T]);
      setErrors(prev => ({ ...prev, [name]: fieldErrors }));
    },
    [validateField, values]
  );

  const validateForm = useCallback(() => {
    const newErrors: Errors = {};
    let isValid = true;

    Object.keys(validationRules).forEach(fieldName => {
      const fieldErrors = validateField(fieldName, values[fieldName as keyof T]);
      if (fieldErrors.length > 0) {
        newErrors[fieldName] = fieldErrors;
        isValid = false;
      }
    });

    setErrors(newErrors);
    return isValid;
  }, [validateField, values, validationRules]);

  return {
    values,
    errors,
    touched,
    handleChange,
    handleBlur,
    validateForm,
    setValues,
  };
}

export default useFormValidation;
