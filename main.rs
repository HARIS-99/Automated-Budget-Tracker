use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::Path;

#[derive(Debug)]
struct Transaction {
    amount: f64,
    source_or_category: String,
    date: String,
}

struct Budget {
    total_budget: f64,
    income: Vec<Transaction>,
    expenses: Vec<Transaction>,
}

impl Budget {
    fn new(total_budget: f64) -> Self {
        Budget {
            total_budget,
            income: Vec::new(),
            expenses: Vec::new(),
        }
    }

    fn add_income(&mut self, amount: f64, source: String, date: String) {
        self.income.push(Transaction {
            amount,
            source_or_category: source,
            date,
        });
    }

    fn add_expense(&mut self, amount: f64, category: String, date: String) {
        self.expenses.push(Transaction {
            amount,
            source_or_category: category,
            date,
        });
    }

    fn display(&self) {
        println!("\n--- Income ---");
        if self.income.is_empty() {
            println!("No income records.");
        } else {
            for (i, inc) in self.income.iter().enumerate() {
                println!("{}: ${:.2} from '{}' on {}", i + 1, inc.amount, inc.source_or_category, inc.date);
            }
        }

        println!("\n--- Expenses ---");
        if self.expenses.is_empty() {
            println!("No expense records.");
        } else {
            for (i, exp) in self.expenses.iter().enumerate() {
                println!("{}: ${:.2} for '{}' on {}", i + 1, exp.amount, exp.source_or_category, exp.date);
            }
        }

        let total_income: f64 = self.income.iter().map(|x| x.amount).sum();
        let total_expenses: f64 = self.expenses.iter().map(|x| x.amount).sum();

        println!("\nTotal Income: ${:.2}", total_income);
        println!("Total Expenses: ${:.2}", total_expenses);
        println!("Remaining Budget: ${:.2}", total_income - total_expenses);
    }

    fn check_budget_overrun(&self) {
        let total_expenses: f64 = self.expenses.iter().map(|x| x.amount).sum();
        if total_expenses > self.total_budget {
            println!("Warning: You have exceeded your budget! Total Expenses: ${:.2}, Budget: ${:.2}", total_expenses, self.total_budget);
        } else {
            println!("Your expenses are within the budget. Total Expenses: ${:.2}, Budget: ${:.2}", total_expenses, self.total_budget);
        }
    }

    fn save_to_csv(&self, path: &str, append: bool) -> io::Result<()> {
        let file_exists = Path::new(path).exists();
        let mut open_options = OpenOptions::new();
        open_options.write(true);

        if append && file_exists {
            open_options.append(true);
        } else {
            open_options.create(true).truncate(true);
        }

        let mut file = open_options.open(path)?;

        // Write headers only if file is new or truncated
        if !append || !file_exists {
            writeln!(file, "Type,Amount,Source/Category,Date")?;
        }

        for inc in &self.income {
            writeln!(file, "Income,{},{},{}", inc.amount, inc.source_or_category, inc.date)?;
        }
        for exp in &self.expenses {
            writeln!(file, "Expense,{},{},{}", exp.amount, exp.source_or_category, exp.date)?;
        }

        Ok(())
    }
}

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn read_f64(prompt: &str) -> f64 {
    loop {
        let input = read_input(prompt);
        match input.parse::<f64>() {
            Ok(num) if num >= 0.0 => return num,
            _ => println!("Invalid number, please enter a positive numeric value."),
        }
    }
}

fn read_date(prompt: &str) -> String {
    loop {
        let input = read_input(prompt);
        if input.len() == 10 && input.chars().nth(4) == Some('-') && input.chars().nth(7) == Some('-') {
            return input;
        } else {
            println!("Invalid date format. Please enter in YYYY-MM-DD format.");
        }
    }
}

fn read_yes_no(prompt: &str) -> bool {
    loop {
        let input = read_input(prompt);
        match input.to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => println!("Please enter 'y' or 'n'."),
        }
    }
}

fn select_transaction_type() -> Option<&'static str> {
    loop {
        println!("Select transaction type:");
        println!("1. Income");
        println!("2. Expense");
        let choice = read_input("Enter choice: ");
        match choice.as_str() {
            "1" => return Some("income"),
            "2" => return Some("expense"),
            _ => println!("Invalid option, please enter 1 or 2."),
        }
    }
}

fn main() {
    let mut budget = Budget::new(1000.0); // Initial budget of $1000
    let csv_file = "budget_data.csv";

    loop {
        println!("\nSelect an option:");
        println!("1. Add Income/Budget");
        println!("2. Add Expense");
        println!("3. Display Data");
        println!("4. Save Data to CSV");
        println!("5. Edit Entry");
        println!("6. Remove Entry");
        println!("7. Check if Expenses Exceed Budget"); // New Option
        println!("8. Enter Previous Budget"); // New Option
        println!("9. Exit");

        let choice = read_input("Enter choice: ");

        match choice.as_str() {
            "1" => {
                let amount = read_f64("Enter income amount: ");
                let source = read_input("Enter income source: ");
                let date = read_date("Enter date (YYYY-MM-DD): ");
                budget.add_income(amount, source, date);
                println!("Income added.");
            }
            "2" => {
                let amount = read_f64("Enter expense amount: ");
                let category = read_input("Enter expense category: ");
                let date = read_date("Enter date (YYYY-MM-DD): ");
                budget.add_expense(amount, category, date);
                println!("Expense added.");
            }
            "3" => {
                budget.display();
            }
            "4" => {
                let append = if std::path::Path::new(csv_file).exists() {
                    read_yes_no("CSV file exists. Append data? (y/n): ")
                } else {
                    false
                };

                if read_yes_no("Do you want to save data to CSV? (y/n): ") {
                    match budget.save_to_csv(csv_file, append) {
                        Ok(_) => println!("Data saved successfully to '{}'.", csv_file),
                        Err(e) => println!("Failed to save data: {}", e),
                    }
                } else {
                    println!("Save canceled.");
                }
            }
            "5" => {
                let tx_type = select_transaction_type().unwrap();
                match tx_type {
                    "income" => {
                        if budget.income.is_empty() {
                            println!("No income records to edit.");
                        } else {
                            budget.display();
                            // Add editing logic here
                        }
                    }
                    "expense" => {
                        if budget.expenses.is_empty() {
                            println!("No expense records to edit.");
                        } else {
                            budget.display();
                            // Add editing logic here
                        }
                    }
                    _ => unreachable!(),
                }
            }
            "6" => {
                let tx_type = select_transaction_type().unwrap();
                match tx_type {
                    "income" => {
                        if budget.income.is_empty() {
                            println!("No income records to remove.");
                        } else {
                            budget.display();
                            // Add remove logic here
                        }
                    }
                    "expense" => {
                        if budget.expenses.is_empty() {
                            println!("No expense records to remove.");
                        } else {
                            budget.display();
                            // Add remove logic here
                        }
                    }
                    _ => unreachable!(),
                }
            }
            "7" => {
                budget.check_budget_overrun(); // Budget overrun check
            }
            "8" => {
                let previous_budget = read_f64("Enter your previous budget: ");
                budget.total_budget = previous_budget; // Set the new budget
                println!("Previous budget of ${:.2} set.", previous_budget);
            }
            "9" => {
                println!("Exiting program.");
                break;
            }
            _ => println!("Invalid option, please try again."),
        }
    }
}
