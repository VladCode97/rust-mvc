use std::collections::HashMap;

use crate::{task_model::TaskModel, task_controller::TaskController};

mod task_model;
mod task_controller;

fn main() {
    let mut tasks: HashMap<u32, TaskModel> = HashMap::new();
    TaskController::create_task(&mut tasks,  TaskModel {
        id: 1,
        name: "Play".to_string(),
        description: "Play with my dog".to_string(),
    });
    TaskController::create_task(&mut tasks, TaskModel  {
        id: 2,
        name: "Drive".to_string(),
        description: "I have to drive until Miami".to_string(),
    });
    TaskController::view_tasks(&mut tasks);
}