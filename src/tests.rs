// src/tests.rs
#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::CalculatorApp;
    use crate::calculator::Calculator;

    #[test]
    fn test_calculator_initialization() {
        let calc = CalculatorApp::default();
        assert_eq!(calc.display, "0");
        assert_eq!(calc.current_input, "");
        assert_eq!(calc.last_result, 0.0);
    }

    #[test]
    fn test_basic_arithmetic() {
        let calc = Calculator::new(true);
        
        assert_eq!(calc.evaluate("5+3").unwrap(), 8.0);
        assert_eq!(calc.evaluate("10-4").unwrap(), 6.0);
        assert_eq!(calc.evaluate("8*8").unwrap(), 64.0);
        assert_eq!(calc.evaluate("15/3").unwrap(), 5.0);
    }

    #[test]
    fn test_format_expression() {
        let calc = Calculator::new(true);
        
        assert_eq!(calc.format_expression("5+3"), "5 + 3");
        assert_eq!(calc.format_expression("10*5"), "10 * 5");
        assert_eq!(calc.format_expression("8-2"), "8 - 2");
    }

    #[test]
    fn test_error_handling() {
        let calc = Calculator::new(true);
        
        assert!(calc.evaluate("").is_err());
        assert!(calc.evaluate("abc").is_err());
        assert!(calc.evaluate("5/0").is_err());
    }

    #[test]
    fn test_multiple_operations() {
        let calc = Calculator::new(true);
        
        assert_eq!(calc.evaluate("1+2+3").unwrap(), 6.0);
        assert_eq!(calc.evaluate("10-2-3").unwrap(), 5.0);
        assert_eq!(calc.evaluate("2*3*4").unwrap(), 24.0);
        assert_eq!(calc.evaluate("24/2/3").unwrap(), 4.0);
        assert_eq!(calc.evaluate("1+2*3").unwrap(), 7.0);
    }

    #[test]
    fn test_operator_precedence() {
        let calc = Calculator::new(true);
        assert_eq!(calc.evaluate("32-6*6").unwrap(), -4.0);
        assert_eq!(calc.evaluate("2+3*4").unwrap(), 14.0);
        assert_eq!(calc.evaluate("10/2*5").unwrap(), 25.0);
        assert_eq!(calc.evaluate("2*3+4*5").unwrap(), 26.0);
    }

    #[test]
    fn test_trigonometric_functions() {
        let calc = Calculator::new(false);  // Use degree mode
        
        // Test exact values
        assert_eq!(calc.evaluate("sin90").unwrap(), 1.0);
        assert_eq!(calc.evaluate("sin0").unwrap(), 0.0);
        assert_eq!(calc.evaluate("cos0").unwrap(), 1.0);
        
        // Test with small tolerance for other angles
        assert!((calc.evaluate("sin45").unwrap() - 0.7071067811865476).abs() < 1e-10);
        assert!((calc.evaluate("cos45").unwrap() - 0.7071067811865476).abs() < 1e-10);
    }

    #[test]
    fn test_function_with_operations() {
        let calc = Calculator::new(false);  // Use degree mode for testing
        
        assert!((calc.evaluate("sin90-1").unwrap() - 0.0).abs() < 1e-10);
        assert!((calc.evaluate("cos0+1").unwrap() - 2.0).abs() < 1e-10);
        assert!((calc.evaluate("tan45*2").unwrap() - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_log_functions() {
        let calc = Calculator::new(true);
        assert!((calc.evaluate("ln1").unwrap() - 0.0).abs() < 1e-10);
        assert!((calc.evaluate("log10").unwrap() - 1.0).abs() < 1e-10);
        assert!(calc.evaluate("ln0").is_err());
        assert!(calc.evaluate("log0").is_err());
    }

    #[test]
    fn test_inverse_trig_functions() {
        let calc = Calculator::new(false);  // Use degree mode
        
        assert!((calc.evaluate("asin0").unwrap() - 0.0).abs() < 1e-10);
        assert!((calc.evaluate("acos1").unwrap() - 0.0).abs() < 1e-10);
        assert!((calc.evaluate("atan0").unwrap() - 0.0).abs() < 1e-10);
        assert!(calc.evaluate("asin2").is_err());
        assert!(calc.evaluate("acos-2").is_err());
    }
}
