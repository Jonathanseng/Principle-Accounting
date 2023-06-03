fn accrual_principle(revenue: i32, expenses: i32) -> i32 {
  // Calculates the accruals using the accrual principle.

  // Args:
  //   revenue: The amount of revenue recognized during the period.
  //   expenses: The amount of expenses recognized during the period.

  // Returns:
  //   The amount of accruals.

  let accruals = revenue - expenses;

// This code works by first taking the two arguments, revenue and expenses, and then subtracting them to get the amount of accruals. The amount of accruals is then returned.
  return accruals;
}

fn main() {
  // Set the revenue and expenses.
  let revenue = 1000;
  let expenses = 500;

  // Calculate the accruals.
  let accruals = accrual_principle(revenue, expenses);

  // Print the accruals.
  println!("{}", accruals);
}

// The accrual principle is an accounting principle that requires companies to record expenses when they are incurred, even if they have not yet been paid. This is in contrast to the cash basis of accounting, which records expenses only when they are actually paid. The accrual principle is used to ensure that a company's financial statements accurately reflect its financial performance for the period.
