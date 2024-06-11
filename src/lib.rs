use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Person(&'static str);

pub type CurrencyUnit = i64;

pub struct Transaction {
    pub paid_by: Person,
    pub split_by: Vec<(Person, CurrencyUnit)>,
}

#[derive(Debug, PartialEq)]
pub struct Debt {
    pub amount: CurrencyUnit,
    pub owing: Person,
    pub owed: Person,
}

pub fn simplify(transactions: Vec<Transaction>) -> Vec<Debt> {
    let mut balances: HashMap<Person, CurrencyUnit> = HashMap::new();

    // Calculate the net balance for each person
    for transaction in transactions.iter() {
        let total = transaction
            .split_by
            .iter()
            .map(|(_, amount)| amount)
            .sum::<CurrencyUnit>();

        // Credit the person who paid
        *balances.entry(transaction.paid_by).or_default() += total;

        // Debit the people who owe
        for (person, amount) in transaction.split_by.iter() {
            *balances.entry(*person).or_default() -= amount;
        }
    }

    // Sort the balances by the amount owing
    let mut sorted_balances: BTreeMap<CurrencyUnit, Person> = balances
        .into_iter()
        .filter_map(|(person, amount)| {
            if amount != 0 {
                Some((amount, person))
            } else {
                None
            }
        })
        .collect();

    let mut debts = vec![];

    // Pop the smallest and largest balances and settle them, adding any remaining balance back to the sorted balances
    while sorted_balances.len() > 1 {
        let (min_amount, min_person) = sorted_balances.pop_first().unwrap();
        let (max_amount, max_person) = sorted_balances.pop_last().unwrap();

        let amount = min_amount.abs().min(max_amount.abs());
        let debt = Debt {
            amount,
            owing: min_person,
            owed: max_person,
        };

        if min_amount.abs() > max_amount.abs() {
            sorted_balances.insert(min_amount + max_amount, min_person);
        } else if max_amount.abs() > min_amount.abs() {
            sorted_balances.insert(min_amount + max_amount, max_person);
        }

        debts.push(debt);
    }

    debts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simplify() {
        let transactions = vec![
            Transaction {
                paid_by: Person("A"),
                split_by: vec![(Person("A"), 50), (Person("B"), 50)],
            },
            Transaction {
                paid_by: Person("B"),
                split_by: vec![(Person("A"), 25), (Person("B"), 25)],
            },
            Transaction {
                paid_by: Person("C"),
                split_by: vec![(Person("A"), 100), (Person("B"), 150), (Person("C"), 50)],
            },
            Transaction {
                paid_by: Person("D"),
                split_by: vec![(Person("D"), 10), (Person("E"), 10)],
            },
            Transaction {
                paid_by: Person("A"),
                split_by: vec![(Person("A"), 5), (Person("E"), 15), (Person("C"), 20)],
            },
        ];

        let debts = simplify(transactions);
        assert_eq!(
            debts,
            vec![
                Debt {
                    amount: 175,
                    owing: Person("B"),
                    owed: Person("C")
                },
                Debt {
                    amount: 40,
                    owing: Person("A"),
                    owed: Person("C")
                },
                Debt {
                    amount: 15,
                    owing: Person("E"),
                    owed: Person("C")
                },
                Debt {
                    amount: 10,
                    owing: Person("E"),
                    owed: Person("D")
                }
            ]
        );
    }
}
