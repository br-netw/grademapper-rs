use iced::Settings;
use iced::pure::Sandbox;
use iced::pure::widget::{Row, Button, Text, TextInput, Column, Container};

fn main() -> Result<(), iced::Error> {
    GradeMapper::run(Settings::default())
}

#[derive(Debug, Clone)]
enum Message {
    EditGrade(String),
    ProcessGrade(f32),
    RemoveGrade,
}

struct GradeMapper {
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

    fn view(&self) -> iced::pure::Element<Self::Message> {
        let avg_out = Text::new(format!("BALL: {}", self.avg));

        // кириллица не работает
        let work1 = Button::new("KLASSNAYA").on_press(Message::ProcessGrade(1.0));
        let work2 = Button::new("SAMOSTOYATELNAYA").on_press(Message::ProcessGrade(1.2));
        let work3 = Button::new("PROVEROCHNAYA").on_press(Message::ProcessGrade(1.3));
        let work4 = Button::new("KONTROLNAYA").on_press(Message::ProcessGrade(1.5));
        
        let rm_button = Button::new("DELETE LAST GRADE").on_press(Message::RemoveGrade);
        let grade_in = TextInput::new("ENTER GRADE", &self.current_grade, Message::EditGrade);

        let main_row = Column::new()
            .push(Row::new().push(grade_in).spacing(20))
            .push(Row::new().push(avg_out).push(rm_button).spacing(20))
            .push(Row::new().push(work1).push(work2).push(work3).push(work4).spacing(5))
            .spacing(20);

        let container = Container::new(main_row)
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
