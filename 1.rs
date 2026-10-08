use std::cmp::Ordering;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

const DATA_FILE: &str = "tasks.db";
const VERSION: u32 = 1;

#[derive(Clone, Debug, PartialEq)]
enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

impl Priority {
    fn from_string(value: &str) -> Priority {
        match value.to_lowercase().as_str() {
            "low" => Priority::Low,
            "medium" => Priority::Medium,
            "high" => Priority::High,
            "critical" => Priority::Critical,
            _ => Priority::Medium,
        }
    }

    fn as_string(&self) -> &'static str {
        match self {
            Priority::Low => "Low",
            Priority::Medium => "Medium",
            Priority::High => "High",
            Priority::Critical => "Critical",
        }
    }

    fn weight(&self) -> u8 {
        match self {
            Priority::Low => 1,
            Priority::Medium => 2,
            Priority::High => 3,
            Priority::Critical => 4,
        }
    }

    fn symbol(&self) -> char {
        match self {
            Priority::Low => 'L',
            Priority::Medium => 'M',
            Priority::High => 'H',
            Priority::Critical => 'C',
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Status {
    Pending,
    Completed,
    Cancelled,
}

impl Status {
    fn from_string(value: &str) -> Status {
        match value.to_lowercase().as_str() {
            "pending" => Status::Pending,
            "completed" => Status::Completed,
            "cancelled" => Status::Cancelled,
            _ => Status::Pending,
        }
    }

    fn as_string(&self) -> &'static str {
        match self {
            Status::Pending => "Pending",
            Status::Completed => "Completed",
            Status::Cancelled => "Cancelled",
        }
    }

    fn symbol(&self) -> char {
        match self {
            Status::Pending => ' ',
            Status::Completed => '✓',
            Status::Cancelled => 'X',
        }
    }
}

#[derive(Clone, Debug)]
struct Task {
    id: u64,
    title: String,
    description: String,
    priority: Priority,
    status: Status,
    category: String,
    due_date: Option<String>,
    created_at: u64,
    completed_at: Option<u64>,
}

impl Task {
    fn new(
        id: u64,
        title: String,
        description: String,
        priority: Priority,
        category: String,
        due_date: Option<String>,
    ) -> Task {
        Task {
            id,
            title,
            description,
            priority,
            status: Status::Pending,
            category,
            due_date,
            created_at: current_timestamp(),
            completed_at: None,
        }
    }

    fn is_active(&self) -> bool {
        self.status == Status::Pending
    }

    fn is_completed(&self) -> bool {
        self.status == Status::Completed
    }

    fn is_cancelled(&self) -> bool {
        self.status == Status::Cancelled
    }

    fn mark_completed(&mut self) {
        self.status = Status::Completed;
        self.completed_at = Some(current_timestamp());
    }

    fn mark_pending(&mut self) {
        self.status = Status::Pending;
        self.completed_at = None;
    }

    fn mark_cancelled(&mut self) {
        self.status = Status::Cancelled;
        self.completed_at = None;
    }

    fn short_description(&self) -> String {
        if self.description.is_empty() {
            return String::from("-");
        }

        if self.description.len() <= 45 {
            return self.description.clone();
        }

        format!("{}...", &self.description[..42])
    }

    fn due_text(&self) -> String {
        match &self.due_date {
            Some(date) => date.clone(),
            None => String::from("-"),
        }
    }

    fn to_record(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}",
            self.id,
            encode(&self.title),
            encode(&self.description),
            self.priority.as_string(),
            self.status.as_string(),
            encode(&self.category),
            self.due_date
                .as_ref()
                .map(|x| encode(x))
                .unwrap_or_else(|| String::from("-")),
            self.created_at,
            self.completed_at
                .map(|x| x.to_string())
                .unwrap_or_else(|| String::from("-"))
        )
    }

    fn from_record(line: &str) -> Option<Task> {
        let parts: Vec<&str> = line.split('|').collect();

        if parts.len() != 9 {
            return None;
        }

        let id = parts[0].parse::<u64>().ok()?;
        let created_at = parts[7].parse::<u64>().ok()?;

        let completed_at = if parts[8] == "-" {
            None
        } else {
            Some(parts[8].parse::<u64>().ok()?)
        };

        let due_date = if parts[6] == "-" {
            None
        } else {
            Some(decode(parts[6]))
        };

        Some(Task {
            id,
            title: decode(parts[1]),
            description: decode(parts[2]),
            priority: Priority::from_string(parts[3]),
            status: Status::from_string(parts[4]),
            category: decode(parts[5]),
            due_date,
            created_at,
            completed_at,
        })
    }
}

struct TaskManager {
    tasks: Vec<Task>,
    next_id: u64,
}

impl TaskManager {
    fn new() -> TaskManager {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn load() -> io::Result<TaskManager> {
        if !Path::new(DATA_FILE).exists() {
            return Ok(TaskManager::new());
        }

        let file = File::open(DATA_FILE)?;
        let reader = BufReader::new(file);

        let mut manager = TaskManager::new();

        for line_result in reader.lines() {
            let line = line_result?;

            if line.starts_with("# TASKFLOW") {
                continue;
            }

            if line.trim().is_empty() {
                continue;
            }

            if let Some(task) = Task::from_record(&line) {
                if task.id >= manager.next_id {
                    manager.next_id = task.id + 1;
                }

                manager.tasks.push(task);
            }
        }

        Ok(manager)
    }

    fn save(&self) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(DATA_FILE)?;

        writeln!(file, "# TASKFLOW {}", VERSION)?;

        for task in &self.tasks {
            writeln!(file, "{}", task.to_record())?;
        }

        file.flush()?;

        Ok(())
    }

    fn add_task(
        &mut self,
        title: String,
        description: String,
        priority: Priority,
        category: String,
        due_date: Option<String>,
    ) -> u64 {
        let id = self.next_id;

        let task = Task::new(
            id,
            title,
            description,
            priority,
            category,
            due_date,
        );

        self.tasks.push(task);
        self.next_id += 1;

        id
    }

    fn find_index(&self, id: u64) -> Option<usize> {
        self.tasks.iter().position(|task| task.id == id)
    }

    fn find(&self, id: u64) -> Option<&Task> {
        self.tasks.iter().find(|task| task.id == id)
    }

    fn find_mut(&mut self, id: u64) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|task| task.id == id)
    }

    fn complete(&mut self, id: u64) -> bool {
        match self.find_mut(id) {
            Some(task) => {
                task.mark_completed();
                true
            }
            None => false,
        }
    }

    fn reopen(&mut self, id: u64) -> bool {
        match self.find_mut(id) {
            Some(task) => {
                task.mark_pending();
                true
            }
            None => false,
        }
    }

    fn cancel(&mut self, id: u64) -> bool {
        match self.find_mut(id) {
            Some(task) => {
                task.mark_cancelled();
                true
            }
            None => false,
        }
    }

    fn delete(&mut self, id: u64) -> bool {
        if let Some(index) = self.find_index(id) {
            self.tasks.remove(index);
            true
        } else {
            false
        }
    }

    fn active_count(&self) -> usize {
        self.tasks.iter().filter(|x| x.is_active()).count()
    }

    fn completed_count(&self) -> usize {
        self.tasks.iter().filter(|x| x.is_completed()).count()
    }

    fn cancelled_count(&self) -> usize {
        self.tasks.iter().filter(|x| x.is_cancelled()).count()
    }

    fn category_count(&self, category: &str) -> usize {
        self.tasks
            .iter()
            .filter(|task| task.category.eq_ignore_ascii_case(category))
            .count()
    }

    fn categories(&self) -> Vec<String> {
        let mut categories = Vec::new();

        for task in &self.tasks {
            if !task.category.is_empty()
                && !categories
                    .iter()
                    .any(|x: &String| x.eq_ignore_ascii_case(&task.category))
            {
                categories.push(task.category.clone());
            }
        }

        categories.sort();
        categories
    }

    fn search(&self, query: &str) -> Vec<&Task> {
        let query = query.to_lowercase();

        self.tasks
            .iter()
            .filter(|task| {
                task.title.to_lowercase().contains(&query)
                    || task.description.to_lowercase().contains(&query)
                    || task.category.to_lowercase().contains(&query)
            })
            .collect()
    }

    fn sort_by_priority(&mut self) {
        self.tasks.sort_by(|a, b| {
            b.priority
                .weight()
                .cmp(&a.priority.weight())
                .then_with(|| a.id.cmp(&b.id))
        });
    }

    fn sort_by_title(&mut self) {
        self.tasks.sort_by(|a, b| {
            a.title
                .to_lowercase()
                .cmp(&b.title.to_lowercase())
        });
    }

    fn sort_by_date(&mut self) {
        self.tasks.sort_by(|a, b| {
            match (&a.due_date, &b.due_date) {
                (Some(x), Some(y)) => x.cmp(y),
                (Some(_), None) => Ordering::Less,
                (None, Some(_)) => Ordering::Greater,
                (None, None) => a.id.cmp(&b.id),
            }
        });
    }

    fn clear_completed(&mut self) -> usize {
        let before = self.tasks.len();

        self.tasks.retain(|task| !task.is_completed());

        before - self.tasks.len()
    }
}

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn encode(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('|', "\\p")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}

fn decode(value: &str) -> String {
    let mut result = String::new();
    let mut escaped = false;

    for ch in value.chars() {
        if escaped {
            match ch {
                'p' => result.push('|'),
                'n' => result.push('\n'),
                'r' => result.push('\r'),
                '\\' => result.push('\\'),
                other => {
                    result.push('\\');
                    result.push(other);
                }
            }

            escaped = false;
        } else if ch == '\\' {
            escaped = true;
        } else {
            result.push(ch);
        }
    }

    if escaped {
        result.push('\\');
    }

    result
}

fn read_line(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().ok();

    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => input.trim().to_string(),
        Err(_) => String::new(),
    }
}

fn read_required(prompt: &str) -> String {
    loop {
        let value = read_line(prompt);

        if !value.trim().is_empty() {
            return value;
        }

        println!("This field cannot be empty.");
    }
}

fn read_u64(prompt: &str) -> Option<u64> {
    let value = read_line(prompt);

    match value.parse::<u64>() {
        Ok(number) => Some(number),
        Err(_) => {
            println!("Please enter a valid number.");
            None
        }
    }
}

fn confirm(prompt: &str) -> bool {
    loop {
        let value = read_line(&format!("{} [y/n]: ", prompt));

        match value.to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => println!("Please answer y or n."),
        }
    }
}

fn choose_priority() -> Priority {
    println!();
    println!("Priority:");
    println!("1. Low");
    println!("2. Medium");
    println!("3. High");
    println!("4. Critical");

    loop {
        match read_line("Choose priority: ").as_str() {
            "1" => return Priority::Low,
            "2" => return Priority::Medium,
            "3" => return Priority::High,
            "4" => return Priority::Critical,
            _ => println!("Invalid choice."),
        }
    }
}

fn choose_status() -> Status {
    println!();
    println!("Status:");
    println!("1. Pending");
    println!("2. Completed");
    println!("3. Cancelled");

    loop {
        match read_line("Choose status: ").as_str() {
            "1" => return Status::Pending,
            "2" => return Status::Completed,
            "3" => return Status::Cancelled,
            _ => println!("Invalid choice."),
        }
    }
}

fn pause() {
    let _ = read_line("\nPress Enter to continue...");
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn print_header(title: &str) {
    println!();
    println!("============================================================");
    println!("  {}", title);
    println!("============================================================");
}

fn print_task(task: &Task) {
    let status = task.status.as_string();
    let priority = task.priority.as_string();

    println!("ID          : {}", task.id);
    println!("Title       : {}", task.title);
    println!("Description : {}", task.description);
    println!("Priority    : {} [{}]", priority, task.priority.symbol());
    println!("Status      : {} [{}]", status, task.status.symbol());
    println!(
        "Category    : {}",
        if task.category.is_empty() {
            "-"
        } else {
            &task.category
        }
    );
    println!("Due date    : {}", task.due_text());
    println!("Created     : {}", format_timestamp(task.created_at));

    if let Some(time) = task.completed_at {
        println!("Completed   : {}", format_timestamp(time));
    } else {
        println!("Completed   : -");
    }
}

fn print_table(tasks: &[&Task]) {
    if tasks.is_empty() {
        println!("\nNo tasks found.");
        return;
    }

    println!();
    println!(
        "{:<5} {:<4} {:<11} {:<28} {:<14} {:<12}",
        "ID", "ST", "PRIORITY", "TITLE", "CATEGORY", "DUE"
    );

    println!(
        "{:-<5} {:-<4} {:-<11} {:-<28} {:-<14} {:-<12}",
        "", "", "", "", "", ""
    );

    for task in tasks {
        let title = truncate(&task.title, 27);
        let category = truncate(&task.category, 13);
        let due = truncate(&task.due_text(), 11);

        println!(
            "{:<5} {:<4} {:<11} {:<28} {:<14} {:<12}",
            task.id,
            task.status.symbol(),
            task.priority.as_string(),
            title,
            category,
            due
        );
    }

    println!();
    println!("Total: {}", tasks.len());
}

fn truncate(value: &str, max: usize) -> String {
    if value.chars().count() <= max {
        return value.to_string();
    }

    let result: String = value.chars().take(max.saturating_sub(3)).collect();
    format!("{}...", result)
}

fn format_timestamp(timestamp: u64) -> String {
    if timestamp == 0 {
        return String::from("-");
    }

    let days = timestamp / 86_400;
    let remainder = timestamp % 86_400;

    let hours = remainder / 3_600;
    let minutes = (remainder % 3_600) / 60;
    let seconds = remainder % 60;

    format!(
        "epoch day {} {:02}:{:02}:{:02}",
        days, hours, minutes, seconds
    )
}

fn display_all(manager: &TaskManager) {
    print_header("ALL TASKS");

    let mut tasks: Vec<&Task> = manager.tasks.iter().collect();

    tasks.sort_by(|a, b| a.id.cmp(&b.id));

    print_table(&tasks);
}

fn display_pending(manager: &TaskManager) {
    print_header("PENDING TASKS");

    let tasks: Vec<&Task> = manager
        .tasks
        .iter()
        .filter(|task| task.status == Status::Pending)
        .collect();

    print_table(&tasks);
}

fn display_completed(manager: &TaskManager) {
    print_header("COMPLETED TASKS");

    let tasks: Vec<&Task> = manager
        .tasks
        .iter()
        .filter(|task| task.status == Status::Completed)
        .collect();

    print_table(&tasks);
}

fn display_cancelled(manager: &TaskManager) {
    print_header("CANCELLED TASKS");

    let tasks: Vec<&Task> = manager
        .tasks
        .iter()
        .filter(|task| task.status == Status::Cancelled)
        .collect();

    print_table(&tasks);
}

fn add_task_menu(manager: &mut TaskManager) {
    print_header("ADD TASK");

    let title = read_required("Title: ");
    let description = read_line("Description: ");
    let priority = choose_priority();
    let category = read_line("Category: ");

    let due_input = read_line("Due date (YYYY-MM-DD, optional): ");

    let due_date = if due_input.trim().is_empty() {
        None
    } else {
        Some(due_input)
    };

    let id = manager.add_task(
        title,
        description,
        priority,
        category,
        due_date,
    );

    match manager.save() {
        Ok(_) => {
            println!();
            println!("Task created successfully.");
            println!("Task ID: {}", id);
        }
        Err(error) => {
            println!("Task created in memory, but save failed: {}", error);
        }
    }
}

fn view_task_menu(manager: &TaskManager) {
    print_header("VIEW TASK");

    let id = match read_u64("Task ID: ") {
        Some(id) => id,
        None => return,
    };

    match manager.find(id) {
        Some(task) => {
            println!();
            print_task(task);
        }
        None => println!("Task {} does not exist.", id),
    }
}

fn complete_task_menu(manager: &mut TaskManager) {
    print_header("COMPLETE TASK");

    let id = match read_u64("Task ID: ") {
        Some(id) => id,
        None => return,
    };

    match manager.find(id) {
        Some(task) if task.is_completed() => {
            println!("Task is already completed.");
            return;
        }
        Some(_) => {}
        None => {
            println!("Task not found.");
            return;
        }
    }

    if manager.complete(id) {
        match manager.save() {
            Ok(_) => println!("Task {} marked as completed.", id),
            Err(error) => println!("Could not save changes: {}", error),
        }
    }
}

fn reopen_task_menu(manager: &mut TaskManager) {
    print_header("REOPEN TASK");

    let id = match read_u64("Task ID: ") {
        Some(id) => id,
        None => return,
    };

    if manager.reopen(id) {
        match manager.save() {
            Ok(_) => println!("Task {} is pending again.", id),
            Err(error) => println!("Could not save changes: {}", error),
        }
    } else {
        println!("Task {} not found.", id);
    }
}

fn cancel_task_menu(manager: &mut TaskManager) {
    print_header("CANCEL TASK");

    let id = match read_u64("Task ID: ") {
        Some(id) => id,
        None => return,
    };

    if manager.cancel(id) {
        match manager.save() {
            Ok(_) => println!("Task {} cancelled.", id),
            Err(error) => println!("Could not save changes: {}", error),
        }
    } else {
        println!("Task {} not found.", id);
    }
}

fn delete_task_menu(manager: &mut TaskManager) {
    print_header("DELETE TASK");

    let id = match read_u64("Task ID: ") {
        Some(id) => id,
        None => return,
    };

    match manager.find(id) {
        Some(task) => {
            println!("You are deleting:");
            println!("  {} - {}", task.id, task.title);
        }
        None => {
            println!("Task not found.");
            return;
        }
    }

    if !confirm("Delete this task?") {
        println!("Deletion cancelled.");
        return;
    }

    if manager.delete(id) {
        match manager.save() {
            Ok(_) => println!("Task deleted."),
            Err(error) => println!("Could not save changes: {}", error),
        }
    }
}

fn edit_task_menu(manager: &mut TaskManager) {
    print_header("EDIT TASK");

    let id = match read_u64("Task ID: ") {
        Some(id) => id,
        None => return,
    };

    let task = match manager.find(id) {
        Some(task) => task.clone(),
        None => {
            println!("Task not found.");
            return;
        }
    };

    println!();
    println!("Press Enter to keep the current value.");
    println!();

    println!("Current title: {}", task.title);
    let title_input = read_line("New title: ");

    println!("Current description: {}", task.description);
    let description_input = read_line("New description: ");

    println!("Current category: {}", task.category);
    let category_input = read_line("New category: ");

    println!(
        "Current due date: {}",
        task.due_date.as_deref().unwrap_or("-")
    );

    let due_input = read_line("New due date: ");

    println!("Current priority: {}", task.priority.as_string());
    println!("Change priority?");

    let change_priority = confirm("Change priority");

    let new_priority = if change_priority {
        Some(choose_priority())
    } else {
        None
    };

    if let Some(target) = manager.find_mut(id) {
        if !title_input.trim().is_empty() {
            target.title = title_input;
        }

        if !description_input.trim().is_empty() {
            target.description = description_input;
        }

        if !category_input.trim().is_empty() {
            target.category = category_input;
        }

        if !due_input.trim().is_empty() {
            target.due_date = Some(due_input);
        }

        if let Some(priority) = new_priority {
            target.priority = priority;
        }
    }

    match manager.save() {
        Ok(_) => println!("Task updated successfully."),
        Err(error) => println!("Could not save changes: {}", error),
    }
}

fn search_menu(manager: &TaskManager) {
    print_header("SEARCH TASKS");

    let query = read_required("Search: ");

    let results = manager.search(&query);

    print_table(&results);
}

fn category_menu(manager: &TaskManager) {
    print_header("CATEGORY FILTER");

    let categories = manager.categories();

    if categories.is_empty() {
        println!("No categories exist.");
        return;
    }

    println!("Available categories:");

    for (index, category) in categories.iter().enumerate() {
        println!(
            "{}. {} ({})",
            index + 1,
            category,
            manager.category_count(category)
        );
    }

    let choice = match read_u64("Choose category number: ") {
        Some(value) => value,
        None => return,
    };

    if choice == 0 || choice as usize > categories.len() {
        println!("Invalid category.");
        return;
    }

    let category = &categories[choice as usize - 1];

    let tasks: Vec<&Task> = manager
        .tasks
        .iter()
        .filter(|task| task.category.eq_ignore_ascii_case(category))
        .collect();

    println!();
    println!("Category: {}", category);

    print_table(&tasks);
}

fn priority_menu(manager: &TaskManager) {
    print_header("PRIORITY FILTER");

    let priority = choose_priority();

    let tasks: Vec<&Task> = manager
        .tasks
        .iter()
        .filter(|task| task.priority == priority)
        .collect();

    print_table(&tasks);
}

fn status_menu(manager: &TaskManager) {
    print_header("STATUS FILTER");

    let status = choose_status();

    let tasks: Vec<&Task> = manager
        .tasks
        .iter()
        .filter(|task| task.status == status)
        .collect();

    print_table(&tasks);
}

fn sort_menu(manager: &mut TaskManager) {
    print_header("SORT TASKS");

    println!("1. Sort by priority");
    println!("2. Sort by title");
    println!("3. Sort by due date");

    match read_line("Choose: ").as_str() {
        "1" => {
            manager.sort_by_priority();
            println!("Tasks sorted by priority.");
        }
        "2" => {
            manager.sort_by_title();
            println!("Tasks sorted by title.");
        }
        "3" => {
            manager.sort_by_date();
            println!("Tasks sorted by due date.");
        }
        _ => {
            println!("Invalid option.");
            return;
        }
    }

    if let Err(error) = manager.save() {
        println!("Could not save sorting order: {}", error);
    }
}

fn statistics_menu(manager: &TaskManager) {
    print_header("STATISTICS");

    let total = manager.tasks.len();
    let pending = manager.active_count();
    let completed = manager.completed_count();
    let cancelled = manager.cancelled_count();

    println!("Total tasks     : {}", total);
    println!("Pending tasks   : {}", pending);
    println!("Completed tasks : {}", completed);
    println!("Cancelled tasks : {}", cancelled);

    println!();

    if total > 0 {
        let completion_rate = completed as f64 / total as f64 * 100.0;

        println!(
            "Completion rate : {:.1}%",
            completion_rate
        );
    } else {
        println!("Completion rate : 0.0%");
    }

    println!();
    println!("Priority distribution:");

    let low = manager
        .tasks
        .iter()
        .filter(|task| task.priority == Priority::Low)
        .count();

    let medium = manager
        .tasks
        .iter()
        .filter(|task| task.priority == Priority::Medium)
        .count();

    let high = manager
        .tasks
        .iter()
        .filter(|task| task.priority == Priority::High)
        .count();

    let critical = manager
        .tasks
        .iter()
        .filter(|task| task.priority == Priority::Critical)
        .count();

    println!("  Low      : {}", low);
    println!("  Medium   : {}", medium);
    println!("  High     : {}", high);
    println!("  Critical : {}", critical);

    println!();
    println!("Categories:");

    for category in manager.categories() {
        println!(
            "  {:<20} {} task(s)",
            category,
            manager.category_count(&category)
        );
    }
}

fn clear_completed_menu(manager: &mut TaskManager) {
    print_header("CLEAR COMPLETED TASKS");

    let count = manager.completed_count();

    if count == 0 {
        println!("There are no completed tasks.");
        return;
    }

    println!("Completed tasks: {}", count);

    if !confirm("Permanently delete all completed tasks?") {
        println!("Operation cancelled.");
        return;
    }

    let removed = manager.clear_completed();

    match manager.save() {
        Ok(_) => println!("Removed {} completed task(s).", removed),
        Err(error) => println!("Removed tasks but save failed: {}", error),
    }
}

fn backup_database() {
    print_header("BACKUP DATABASE");

    if !Path::new(DATA_FILE).exists() {
        println!("No database exists yet.");
        return;
    }

    let timestamp = current_timestamp();
    let backup_name = format!("tasks_backup_{}.db", timestamp);

    match fs::copy(DATA_FILE, &backup_name) {
        Ok(bytes) => {
            println!("Backup created.");
            println!("File: {}", backup_name);
            println!("Bytes copied: {}", bytes);
        }
        Err(error) => {
            println!("Backup failed: {}", error);
        }
    }
}

fn import_database(manager: &mut TaskManager) {
    print_header("IMPORT DATABASE");

    let path = read_required("Database file path: ");

    if !Path::new(&path).exists() {
        println!("File does not exist.");
        return;
    }

    let file = match File::open(&path) {
        Ok(file) => file,
        Err(error) => {
            println!("Could not open file: {}", error);
            return;
        }
    };

    let reader = BufReader::new(file);
    let mut imported = 0usize;

    for line_result in reader.lines() {
        let line = match line_result {
            Ok(line) => line,
            Err(_) => continue,
        };

        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }

        if let Some(mut task) = Task::from_record(&line) {
            if manager.find(task.id).is_some() {
                task.id = manager.next_id;
            }

            if task.id >= manager.next_id {
                manager.next_id = task.id + 1;
            }

            manager.tasks.push(task);
            imported += 1;
        }
    }

    match manager.save() {
        Ok(_) => println!("Imported {} task(s).", imported),
        Err(error) => println!("Import succeeded but save failed: {}", error),
    }
}

fn export_database(manager: &TaskManager) {
    print_header("EXPORT DATABASE");

    let path = read_required("Export file path: ");

    let mut file = match File::create(&path) {
        Ok(file) => file,
        Err(error) => {
            println!("Could not create file: {}", error);
            return;
        }
    };

    if let Err(error) = writeln!(file, "# TASKFLOW {}", VERSION) {
        println!("Could not write export: {}", error);
        return;
    }

    for task in &manager.tasks {
        if let Err(error) = writeln!(file, "{}", task.to_record()) {
            println!("Could not write task: {}", error);
            return;
        }
    }

    println!("Exported {} task(s).", manager.tasks.len());
    println!("File: {}", path);
}

fn show_help() {
    print_header("HELP");

    println!("TaskFlow is a command-line task management application.");
    println!();
    println!("Main concepts:");
    println!("  Task       A unit of work.");
    println!("  Priority   Low, Medium, High, or Critical.");
    println!("  Status     Pending, Completed, or Cancelled.");
    println!("  Category   A custom label for grouping tasks.");
    println!("  Due date   Optional YYYY-MM-DD value.");
    println!();
    println!("Data storage:");
    println!("  Tasks are automatically stored in tasks.db.");
    println!("  Changes are saved after modification.");
    println!();
    println!("Search:");
    println!("  Searches title, description, and category.");
    println!();
    println!("Sorting:");
    println!("  Priority puts important tasks first.");
    println!("  Title sorts alphabetically.");
    println!("  Due date puts dated tasks first.");
    println!();
    println!("Tip:");
    println!("  Use categories such as Work, Study, Personal, or Finance.");
}

fn print_menu(manager: &TaskManager) {
    println!();
    println!("============================================================");
    println!("                       TASKFLOW");
    println!("============================================================");
    println!(
        " Pending: {} | Completed: {} | Cancelled: {}",
        manager.active_count(),
        manager.completed_count(),
        manager.cancelled_count()
    );
    println!("------------------------------------------------------------");
    println!(" 1.  Add task");
    println!(" 2.  View all tasks");
    println!(" 3.  View pending tasks");
    println!(" 4.  View completed tasks");
    println!(" 5.  View cancelled tasks");
    println!(" 6.  View task details");
    println!(" 7.  Edit task");
    println!(" 8.  Complete task");
    println!(" 9.  Reopen task");
    println!("10.  Cancel task");
    println!("11.  Delete task");
    println!("12.  Search tasks");
    println!("13.  Filter by category");
    println!("14.  Filter by priority");
    println!("15.  Filter by status");
    println!("16.  Sort tasks");
    println!("17.  Statistics");
    println!("18.  Clear completed");
    println!("19.  Backup database");
    println!("20.  Import database");
    println!("21.  Export database");
    println!("22.  Help");
    println!(" 0.  Exit");
    println!("------------------------------------------------------------");
}

fn handle_menu(manager: &mut TaskManager, choice: &str) -> bool {
    match choice {
        "1" => {
            add_task_menu(manager);
        }
        "2" => {
            display_all(manager);
        }
        "3" => {
            display_pending(manager);
        }
        "4" => {
            display_completed(manager);
        }
        "5" => {
            display_cancelled(manager);
        }
        "6" => {
            view_task_menu(manager);
        }
        "7" => {
            edit_task_menu(manager);
        }
        "8" => {
            complete_task_menu(manager);
        }
        "9" => {
            reopen_task_menu(manager);
        }
        "10" => {
            cancel_task_menu(manager);
        }
        "11" => {
            delete_task_menu(manager);
        }
        "12" => {
            search_menu(manager);
        }
        "13" => {
            category_menu(manager);
        }
        "14" => {
            priority_menu(manager);
        }
        "15" => {
            status_menu(manager);
        }
        "16" => {
            sort_menu(manager);
        }
        "17" => {
            statistics_menu(manager);
        }
        "18" => {
            clear_completed_menu(manager);
        }
        "19" => {
            backup_database();
        }
        "20" => {
            import_database(manager);
        }
        "21" => {
            export_database(manager);
        }
        "22" => {
            show_help();
        }
        "0" => {
            return false;
        }
        _ => {
            println!("Invalid menu option.");
        }
    }

    true
}

fn startup_message(manager: &TaskManager) {
    clear_screen();

    println!("============================================================");
    println!("                 Welcome to TaskFlow");
    println!("============================================================");
    println!();
    println!("A Rust command-line task manager.");
    println!();
    println!("Loaded {} task(s).", manager.tasks.len());

    if manager.tasks.is_empty() {
        println!("Your task database is currently empty.");
    } else {
        println!(
            "{} pending, {} completed, {} cancelled.",
            manager.active_count(),
            manager.completed_count(),
            manager.cancelled_count()
        );
    }

    println!();
}

fn load_manager() -> TaskManager {
    match TaskManager::load() {
        Ok(manager) => manager,
        Err(error) => {
            println!("Could not load database: {}", error);
            println!("Starting with an empty database.");
            TaskManager::new()
        }
    }
}

fn main() {
    let mut manager = load_manager();

    startup_message(&manager);
    pause();

    loop {
        clear_screen();

        print_menu(&manager);

        let choice = read_line("Choose an option: ");

        if !handle_menu(&mut manager, &choice) {
            break;
        }

        pause();
    }

    println!();
    println!("Saving database...");

    match manager.save() {
        Ok(_) => println!("Database saved successfully."),
        Err(error) => println!("Could not save database: {}", error),
    }

    println!("Goodbye!");
}
