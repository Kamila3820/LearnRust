// struct QustNotifier;

// impl QustNotifier {
//     fn notify(&self, message: &str) {
//         println!("You receive {}", message);
//     }
// }

// struct QuestMessage {
//     notifier: QustNotifier,
// }

// impl QuestMessage {
//     fn complete_quest(&self) {
//         self.notifier.notify("Quest completed");
//     }
// }

trait QustNotifier{
    fn notify(&self, message: &str) {
        println!("You receive {}", message);
    }
} 

struct Firebase;
struct PostgreSQL;

impl QustNotifier for Firebase {
    fn notify(&self, message: &str) {
        println!("Firebase: {}", message);
    }
}

impl QustNotifier for PostgreSQL {
    fn notify(&self, message: &str) {
        println!("PostgreSQL: {}", message);
    }
}

struct QuestMessage;

impl QuestMessage {
    fn complete_quest<T: QustNotifier>(&self, notifier: T) {
        notifier.notify("Quest Completed !!");
    }
}

fn main() {
    let admin = QuestMessage;
    let firebase = Firebase;
    let postgres = PostgreSQL;

    admin.complete_quest(firebase);
}
