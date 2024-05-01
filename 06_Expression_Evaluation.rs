/// An operation to perform on two subexpressions.
#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

/// An expression, in tree form.
#[derive(Debug)]
enum Expression {
    /// An operation on two subexpressions.
    Op { op: Operation, left: Box<Expression>, right: Box<Expression> },

    /// A literal value
    Value(i64),
}

fn eval(e: Expression) -> Result<i64, String> {
    
    match e {
      // Base case
      Expression::Value(x) => Ok(x),
      // Non base cases
      Expression::Op{op: Operation::Add, left, right} => {
          // Evaluate left and right expressions if not proceed and do addition
          let left_result = eval(*left);
          let right_result = eval(*right);
          
          //Check for any errors and return Err if that is the case
          if let Err(msg) = left_result {
              return Err(msg)
          }
          if let Err(msg) = right_result {
              return Err(msg)
          }
          
          // if not proceed and do addition
          Ok(left_result.unwrap() + right_result.unwrap())
      },
      Expression::Op{op: Operation::Sub, left, right} => {
          // Evaluate left and right expressions if not proceed and do addition
          let left_result = eval(*left);
          let right_result = eval(*right);
          
          //Check for any errors and return Err if that is the case
          if let Err(msg) = left_result {
              return Err(msg)
          }
          if let Err(msg) = right_result {
              return Err(msg)
          }
          
          // if not proceed and do subtraction
          Ok(left_result.unwrap() - right_result.unwrap())
      },
      Expression::Op{op: Operation::Mul, left, right} => {
          // Evaluate left and right expressions if not proceed and do addition
          let left_result = eval(*left);
          let right_result = eval(*right);
          
          //Check for any errors and return Err if that is the case
          if let Err(msg) = left_result {
              return Err(msg)
          }
          if let Err(msg) = right_result {
              return Err(msg)
          }
          
          // if not proceed and do multiplication
          Ok(left_result.unwrap() * right_result.unwrap())
      },
      Expression::Op{op: Operation::Div, left, right} => {
          // Evaluate left and right expressions if not proceed and do addition
          let left_result = eval(*left);
          let right_result = eval(*right);
          
          //Check for any errors and return Err if that is the case
          if let Err(msg) = left_result {
              return Err(msg)
          }
          if let Err(msg) = right_result {
              return Err(msg)
          }
          
          // if not proceed and do division (check for divide by zero error as well)
          // * and as_ref() used to avoid move during unwrap() and deref the returned reference to result
          if *right_result.as_ref().unwrap() == 0 {
              return Err(String::from("division by zero"));
          }
          Ok(left_result.unwrap() / right_result.unwrap())
      }
    }
    
}

#[test]
fn test_value() {
    assert_eq!(eval(Expression::Value(19)), Ok(19));
}

#[test]
fn test_sum() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(Expression::Value(10)),
            right: Box::new(Expression::Value(20)),
        }),
        Ok(30)
    );
}

#[test]
fn test_recursion() {
    let term1 = Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Value(10)),
        right: Box::new(Expression::Value(9)),
    };
    let term2 = Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Op {
            op: Operation::Sub,
            left: Box::new(Expression::Value(3)),
            right: Box::new(Expression::Value(4)),
        }),
        right: Box::new(Expression::Value(5)),
    };
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(term1),
            right: Box::new(term2),
        }),
        Ok(85)
    );
}

#[test]
fn test_error() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Div,
            left: Box::new(Expression::Value(99)),
            right: Box::new(Expression::Value(0)),
        }),
        Err(String::from("division by zero"))
    );
}
