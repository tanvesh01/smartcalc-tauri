use core::cell::{Cell, RefCell};
use alloc::string::{String, ToString};

use alloc::{rc::Rc, vec::Vec};
use regex::Regex;

use crate::variable::VariableInfo;

#[derive(Default)]
pub struct Session {
    text: String,
    text_parts: Vec<String>,
    language: String,
    position: Cell<usize>,

    pub(crate) variables: RefCell<Vec<Rc<VariableInfo>>>
}

impl Session {
    /// Create an empty session.
    ///
    /// In order to be executed, a session must have a language and some text.
    pub fn new() -> Session {
        Session {
            text: String::new(),
            text_parts: Vec::new(),
            language: String::new(),
            variables: RefCell::new(Vec::new()),
            position: Cell::default()
        }
    }

    /// Set the text to be executed.
    pub fn set_text(&mut self, text: String) {
        self.text = text;
        
        self.text_parts = match Regex::new(r"\r\n|\n") {
            Ok(re) => re.split(&self.text).map(|item| item.to_string()).collect::<Vec<_>>(),
            _ => self.text.lines().map(|item| item.to_string()).collect::<Vec<_>>()
        };
    }

    /// Set the language used to interpret input.
    pub fn set_language(&mut self, language: String) {
        self.language = language;
    }
    
    pub(crate) fn current_line(&self) -> &'_ String { 
        &self.text_parts[self.position.get()]
    }
    
    pub(crate) fn has_value(&self) -> bool { 
        self.text_parts.len() > self.position.get()
    }
    
    pub(crate) fn next_line(&self) -> Option<&'_ String> {
        match self.text_parts.len() > self.position.get() + 1 {
            true => {
                let current = Some(self.current_line());
                self.position.set(self.position.get() + 1);
                current
            }
            false => None
        }
    }
    
    pub(crate) fn add_variable(&self, variable_info: Rc<VariableInfo>) {
        self.variables.borrow_mut().push(variable_info);
    }
    
    /// Returns the language configured for this session.
    pub fn get_language(&self) -> String {
        self.language.to_string()
    }
}
