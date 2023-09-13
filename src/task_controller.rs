use crate::task_model::TaskModel;
use std::collections::HashMap;

pub struct TaskController {}

impl TaskController {
    pub fn create_task(tasks: &mut HashMap<u32, TaskModel>, task: TaskModel) {
        tasks.insert(
            task.id,
            TaskModel {
                id: task.id,
                name: task.name,
                description: task.description,
            },
        );
    }

    pub fn view_tasks(tasks: &mut HashMap<u32, TaskModel>) {
        for task in tasks.iter() {
            dbg!(&task);
        }
    }
}
