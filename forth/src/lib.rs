use std::collections::HashMap;

pub type Value = i32;
pub type ForthResult = Result<(), Error>;

pub struct Forth {
    stack: Vec<Value>,
    words: HashMap<String, Vec<ForthToken>>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

#[derive(Clone)]
enum ForthToken {
    Number(Value),
    Word(String),
    WordDefBegin,
    WordDefEnd,
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            stack: vec![],
            words: HashMap::new(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    fn stack_pop(&mut self) -> Result<Value, Error> {
        Ok(self.stack.pop().ok_or(Error::StackUnderflow)?)
    }

    fn eval_tokens(&mut self, tokens: &[ForthToken]) -> ForthResult {
        let mut in_word_def = false;
        let mut word_name = None;
        let mut word_body: Vec<ForthToken> = vec![];

        tokens
            .iter()
            .map(|token| {
                use ForthToken::*;
                if in_word_def {
                    if let Some(_) = &word_name {
                        match token {
                            WordDefEnd => {
                                self.words
                                    .insert(word_name.take().unwrap(), word_body.clone());
                                in_word_def = false;
                                word_body.clear();
                                Ok(())
                            }
                            Word(s) => {
                                if let Some(v) = self.words.get(s) {
                                    Ok(v.iter().for_each(|w| word_body.push(w.clone())))
                                } else {
                                    Ok(word_body.push(token.clone()))
                                }
                            }
                            Number(_) => Ok(word_body.push(token.clone())),
                            _ => Err(Error::InvalidWord),
                        }
                    } else {
                        match token {
                            Word(w) => {
                                word_name = Some(w.clone());
                                Ok(())
                            }
                            _ => Err(Error::InvalidWord),
                        }
                    }
                } else {
                    match token {
                        Number(n) => Ok(self.stack.push(*n)),
                        Word(w) => {
                            if let Some(body) = self.words.get(w) {
                                self.eval_tokens(&body.clone())
                            } else {
                                match w.as_str() {
                                    "dup" => {
                                        let n = self.stack_pop()?;
                                        self.stack.push(n);
                                        self.stack.push(n);
                                        Ok(())
                                    }
                                    "drop" => {
                                        let _ = self.stack_pop()?;
                                        Ok(())
                                    }
                                    "swap" => {
                                        let n2 = self.stack_pop()?;
                                        let n1 = self.stack_pop()?;
                                        self.stack.push(n2);
                                        self.stack.push(n1);
                                        Ok(())
                                    }
                                    "over" => {
                                        let n2 = self.stack_pop()?;
                                        let n1 = self.stack_pop()?;
                                        self.stack.push(n1);
                                        self.stack.push(n2);
                                        self.stack.push(n1);
                                        Ok(())
                                    }
                                    "+" => {
                                        let n2 = self.stack_pop()?;
                                        let n1 = self.stack_pop()?;
                                        Ok(self.stack.push(n1 + n2))
                                    }
                                    "-" => {
                                        let n2 = self.stack_pop()?;
                                        let n1 = self.stack_pop()?;
                                        Ok(self.stack.push(n1 - n2))
                                    }
                                    "*" => {
                                        let n2 = self.stack_pop()?;
                                        let n1 = self.stack_pop()?;
                                        Ok(self.stack.push(n1 * n2))
                                    }
                                    "/" => {
                                        let n2 = self.stack_pop()?;
                                        let n1 = self.stack_pop()?;
                                        if n2 == 0 {
                                            Err(Error::DivisionByZero)
                                        } else {
                                            Ok(self.stack.push(n1 / n2))
                                        }
                                    }
                                    _ => Err(Error::UnknownWord),
                                }
                            }
                        }
                        WordDefBegin => {
                            in_word_def = true;
                            Ok(())
                        }
                        WordDefEnd => Err(Error::InvalidWord),
                    }
                }
            })
            .collect::<ForthResult>()
            .and(if in_word_def {
                Err(Error::InvalidWord)
            } else {
                Ok(())
            })
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        let tokens = input
            .split_ascii_whitespace()
            .map(|s| {
                use ForthToken::*;
                match s {
                    ":" => WordDefBegin,
                    ";" => WordDefEnd,
                    _ => s
                        .parse::<Value>()
                        .map_or_else(|_| Word(String::from(s).to_lowercase()), |v| Number(v)),
                }
            })
            .collect::<Vec<_>>();
        self.eval_tokens(&tokens)
    }
}
