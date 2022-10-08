use iced::Settings;
use iced::Sandbox;
use iced::widget::{Row, button, Button, Text, text_input, TextInput, Column, Container};
use iced::Length::FillPortion;

fn main() -> Result<(), iced::Error> {
    let mut gm_settings = Settings::default();
    gm_settings.id = Some(String::from("grademapper-rs"));
    // Шрифт не меняется, но и прога не вылетает
    gm_settings.default_font = Some(b"JetBrainsMono"); 
    GradeMapper::run(gm_settings)
}

#[derive(Debug, Clone)]
enum Message {
    EditGrade(String),
    ProcessGrade(f32),
    RemoveGrade,
}

struct GradeMapper {
    text_inp_state: text_input::State,
    remove_button_state: button::State,
    button1_state: button::State,
    button2_state: button::State,
    button3_state: button::State,
    button4_state: button::State,
    current_grade: String,
    current_weight: f32,
    grades: Vec<i32>,
    weights: Vec<f32>,
    avg: String,
}

trait CustomSandbox: Sandbox {
    fn compute_avg(&mut self);
}

impl Sandbox for GradeMapper {
    type Message = Message;

    fn new() -> Self {
        GradeMapper{
            text_inp_state: text_input::State::new(),
            remove_button_state: button::State::new(),
            button1_state: button::State::new(),
            button2_state: button::State::new(),
            button3_state: button::State::new(),
            button4_state: button::State::new(),
            current_grade: String::new(), 
            current_weight: 0.0,
            grades: Vec::new(), 
            weights: Vec::new(), 
            avg: String::new()
        }
    }

    fn title(&self) -> String {
        String::from("GradeMapper (Rust edition)")
    }

    fn update(&mut self, click: Self::Message) {
        match click {
            Message::ProcessGrade(f) => {
                self.current_weight = f;
                if let Ok(res) = self.current_grade.parse() {
                    if res > 5 || res < 1 { // проверка на валидность оценки
                        self.avg = String::from("ERROR: incorrect grade");
                    } else {
                        self.grades.push(res);
                        self.weights.push(self.current_weight);
                        self.compute_avg();
                    }
                } else {
                    self.avg = String::from("ERROR: not a number"); 
                }
            },
            Message::RemoveGrade => {
                self.weights.remove(self.weights.len()-1);
                self.grades.remove(self.grades.len()-1);
                self.compute_avg(); // пересчитываем
            },
            Message::EditGrade(s) => self.current_grade = s.clone(), 
        }
    }

    fn view(&mut self) -> iced::Element<Self::Message> {
        let avg_out = Text::new(format!("GRADE: {}", self.avg))
            .width(FillPortion(1));

        // кириллица не работает
        let work1 = Button::new(&mut self.button1_state, Text::new("Klassnaya"))
            .on_press(Message::ProcessGrade(1.0)).width(FillPortion(1)).padding(10);
        let work2 = Button::new(&mut self.button2_state, Text::new("Samostoyatelnaya"))
            .on_press(Message::ProcessGrade(1.2)).width(FillPortion(1)).padding(10);
        let work3 = Button::new(&mut self.button3_state, Text::new("Proverochnaya"))
            .on_press(Message::ProcessGrade(1.3)).width(FillPortion(1)).padding(10);
        let work4 = Button::new(&mut self.button4_state, Text::new("Kontrolnaya"))
            .on_press(Message::ProcessGrade(1.5)).width(FillPortion(1)).padding(10);
        
        let rm_button = Button::new(&mut self.remove_button_state, Text::new("DELETE LAST GRADE"))
            .on_press(Message::RemoveGrade).width(FillPortion(1)).padding(10);
        let grade_in = TextInput::new(&mut self.text_inp_state, "ENTER GRADE", &self.current_grade, Message::EditGrade)
            .width(FillPortion(8)).padding(10);

        let main_column = Column::new()
            .push(Row::new().push(avg_out).push(grade_in).spacing(10).padding(10))
            .push(Row::new().push(rm_button).spacing(10).padding(10))
            .push(Row::new().push(work1).push(work2).push(work3).push(work4).spacing(10).padding(10))
            .spacing(10).padding(20);

        let container = Container::new(main_column)
            .center_x().center_y()
            .width(iced::Length::Fill)
            .height(iced::Length::Fill).into();
        return container;
    }
}
impl CustomSandbox for GradeMapper {
    // находим среднее взвешенное
    fn compute_avg(&mut self) {
        let mut sum_grades = 0.0;
        let mut sum_weights = 0.0;
        for i in 0..self.grades.len() {
            sum_grades += self.grades[i] as f32 * self.weights[i];
            sum_weights += self.weights[i];
        }
        let average = sum_grades as f32 / sum_weights;
        self.avg = average.to_string();
    }
}
