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

    pub fn update_task_by_name(tasks: &mut HashMap<u32, TaskModel>, key: u32, name_task: String) {
        tasks.entry(key).and_modify(|task| task.name = name_task.to_string());
    }

    pub fn delete_by_key(tasks: &mut HashMap<u32, TaskModel>, key: u32) {
        tasks.remove(&key);
    } 

    pub fn view_tasks(tasks: &mut HashMap<u32, TaskModel>) {
        for task in tasks.iter() {
            dbg!(&task);
        }
    }
}
