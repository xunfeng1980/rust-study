use rust_bert::pipelines::question_answering::{QaInput, QuestionAnsweringModel};
use std::time::Instant;

fn main() {
    let qa_model = QuestionAnsweringModel::new(Default::default()).unwrap();
    let question = String::from("Where does Amy live ?");
    let context = String::from("Amy lives in Amsterdam");

    let now = Instant::now();
    let answers = qa_model.predict(&[QaInput { question, context }], 1, 32);
    for answer in answers {
        println!("Answer: {:?}", answer);
    }
    println!("执行时间: {}", now.elapsed().as_millis());
}
