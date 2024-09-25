// Define the different tabs
#[derive(Clone, PartialEq, Eq)]
pub enum TabType {
    Baselayer,
    Rollup(String),
    Marketplace,
    Settings,
}

pub struct Tab {
    pub tab_type: TabType,
    pub sidebar_items: Option<Vec<String>>,
    pub title: String,
}

pub struct App {
    tabs: Vec<Tab>,
    current_tab_index: usize,
    sidebar_index: usize,
}

pub enum MenuItemChange {
    Index(usize),
    Increment,
    Decrement,
}

impl From<usize> for MenuItemChange {
    fn from(index: usize) -> Self {
        MenuItemChange::Index(index)
    }
}

impl App {
    pub fn new() -> App {
        App {
            tabs: vec![
                Tab {
                    tab_type: TabType::Baselayer,
                    title: "Baselayer".to_string(),
                    sidebar_items: None,
                },
                Tab {
                    tab_type: TabType::Rollup("Monea Based Rollup".to_string()),
                    title: "Monea Based Rollup".to_string(),
                    sidebar_items: None,
                },
                Tab {
                    tab_type: TabType::Rollup("My Layer3".to_string()),
                    title: "My Layer3".to_string(),
                    sidebar_items: None,
                },
                Tab {
                    tab_type: TabType::Marketplace,
                    title: "Marketplace".to_string(),
                    sidebar_items: None,
                },
                Tab {
                    tab_type: TabType::Settings,
                    title: "Settings".to_string(),
                    sidebar_items: Some(vec![
                        "Account".to_string(),
                        "General".to_string(),
                        "Network".to_string(),
                        "Advanced".to_string(),
                    ]),
                },
            ],
            current_tab_index: 0,
            sidebar_index: 0,
        }
    }

    pub fn get_tabs(&self) -> &Vec<Tab> {
        &self.tabs
    }

    pub fn active_tab(&self) -> &Tab {
        &self.tabs[self.current_tab_index]
    }

    pub fn active_tab_index(&self) -> usize {
        self.current_tab_index
    }

    pub fn active_tab_sidebar_items(&self) -> Option<&Vec<String>> {
        self.active_tab().sidebar_items.as_ref()
    }

    pub fn active_sidebar_item_index(&self) -> usize {
        self.sidebar_index
    }

    pub fn change_active_tab(&mut self, param: impl Into<MenuItemChange>) {
        match param.into() {
            MenuItemChange::Index(index) => {
                self.current_tab_index = index.saturating_sub(1);
            }
            MenuItemChange::Increment => {
                self.current_tab_index = (self.current_tab_index + 1) % self.tabs.len();
            }
            MenuItemChange::Decrement => {
                self.current_tab_index =
                    (self.current_tab_index + self.tabs.len() - 1) % self.tabs.len();
            }
        }
        self.sidebar_index = 0;
    }
    pub fn change_active_sidebar_item(&mut self, param: impl Into<MenuItemChange>) {
        if let Some(sidebar_items) = &self.active_tab().sidebar_items {
            let sidebar_len = sidebar_items.len();
            match param.into() {
                MenuItemChange::Index(index) => {
                    self.sidebar_index = index.saturating_sub(1);
                }
                MenuItemChange::Increment => {
                    self.sidebar_index = (self.sidebar_index + 1) % sidebar_len;
                }
                MenuItemChange::Decrement => {
                    self.sidebar_index = (self.sidebar_index + sidebar_len - 1) % sidebar_len;
                }
            }
        }
    }

    pub fn add_rollup(&mut self, name: String) {
        self.tabs.insert(
            self.tabs.len() - 1,
            Tab {
                title: name.clone(),
                tab_type: TabType::Rollup(name),
                sidebar_items: None,
            },
        );
    }
}
