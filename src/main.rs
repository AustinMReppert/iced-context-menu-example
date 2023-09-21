mod context_menu;
mod context_menu_stylesheet;

use crate::context_menu::{ContextMenu, ContextMenuState, Id};
use crate::context_menu_stylesheet::ContextMenuStyleSheet;
use iced::widget::{Button, Column, Container, Text};
use iced::{executor, Application, Color, Command, Element, Length, Point, Settings, Theme};
use std::cell::RefCell;
use std::rc::Rc;

pub fn main() -> iced::Result {
    HelloWorld::run(Settings::default())
}

#[derive(Debug, Clone)]
pub enum Message {
    DoNothing,
    OnRightClick(Id, Point),
    OnClose(Id),

    Foo,
    Bar,
    Car,
    Dar,
}

struct HelloWorld {
    context_menu_state: Rc<RefCell<ContextMenuState>>,
}

impl Application for HelloWorld {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let context_menu_state = Rc::new(RefCell::new(ContextMenuState::new()));
        (
            HelloWorld {
                context_menu_state: context_menu_state.clone(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Context Menu")
    }

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        match message {
            Message::OnRightClick(id, position) => {
                self.context_menu_state
                    .borrow_mut()
                    .on_right_click(id, position);
            }
            Message::OnClose(_id) => {
                self.context_menu_state.borrow_mut().on_close();
            }
            Message::Foo => {
                println!("Foo");
            }
            Message::Bar => {
                println!("Bar");
            }
            Message::Car => {
                println!("Car");
            }
            Message::Dar => {
                println!("Dar");
            }
            _ => {}
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let id1 = Id::new("context_menu_1");
        let id2 = Id::new("context_menu_2");

        let dropdown = Column::new()
            .push(
                Button::new("Foo")
                    .on_press(Message::Foo)
                    .width(Length::Fill)
                    .style(ContextMenuStyleSheet::new()),
            )
            .push(
                Button::new("Bar")
                    .on_press(Message::Bar)
                    .width(Length::Fill)
                    .style(ContextMenuStyleSheet::new()),
            )
            .width(150);

        let dropdown2 = Column::new()
            .push(
                Button::new("Car")
                    .width(Length::Fill)
                    .on_press(Message::Car)
                    .style(iced::theme::Button::Custom(Box::new(
                        ContextMenuStyleSheet {},
                    ))),
            )
            .push(
                Button::new("Dar")
                    .width(Length::Fill)
                    .on_press(Message::Dar)
                    .style(iced::theme::Button::Custom(Box::new(
                        ContextMenuStyleSheet {},
                    ))),
            )
            .width(150);

        let context_menu_1 = ContextMenu::new(
            id1.clone(),
            self.context_menu_state.borrow().active(),
            Message::OnRightClick,
            Message::OnClose(id1.clone()),
            Text::new("Right Click Me"),
            dropdown,
        );

        let context_menu_2 = ContextMenu::new(
            id2.clone(),
            self.context_menu_state.borrow().active(),
            Message::OnRightClick,
            Message::OnClose(id2.clone()),
            Text::new("Right Click Me"),
            dropdown2,
        );

        let content = Column::with_children(vec![
            Container::new(context_menu_1).width(200).height(200).into(),
            Container::new(context_menu_2).width(200).height(200).into(),
        ]);

        Element::new(content).explain(Color::BLACK)
    }

    fn theme(&self) -> Theme {
        Theme::default()
    }
}
