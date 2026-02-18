use std::io;

fn main() {
    println!("=== Welcome to Mood Bot! ===");
    println!("What is your name?");

    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("error");
    let name = name.trim();

    println!("Hey {}! How are you feeling?", name);
    println!("1. Happy");
    println!("2. Sad");
    println!("3. Tired");
    println!("4. Stressed");
    println!("5. Excited");
    println!("6. Bored");
    println!("7. Anxious");
    println!("8. Lazy");
    println!("9. Agitated");
    println!("10. Sleepy");
    println!("11. Shocked");
    println!("12. Calm");
    println!("13. Moody");
    println!("14. Irritated");
    println!("15. Energetic");
    println!("16. Confused");
    println!("17. Low Energy");
    println!("18. Apathetic");
    println!("19. Feeling Guilty");
    println!("20. Frisky");

    let mut c = String::new();
    io::stdin().read_line(&mut c).expect("error");

    let msg = match c.trim() {
        "1" => "Keep spreading that positivity!✨",
        "2" => "It's okay. Tomorrow is a new day.😌",
        "3" => "Rest up - you have earned it!😉",
        "4" => "One step at a time. You got this!🤗",
        "5" => "Ride that wave! Today is your day!😁",
        "6" => "It's okay to feel bored,it doesn't mean anything is wrong,your mind wants something new!♥️",
        "7" => "Your safe right now 😌 this feeling will pass!",
        "8" => "Your not lazy,your just overwhelmed or unmotivated rn!🫶",
        "9" => "its okay to feel agitated, this moment will settle!🤞",
        "10" => "You've done enough today, its okay to take a nap!😴",
        "11" => "Take a breath,it's okay you will process this!☺️",
        "12" => "Good!your excatly where you need to be!😎",
        "13" => "Your mood doesn;t define you, it's just a passing mood!😘",
        "14" => "Take a deep breath.This irritation won't last!😚",
        "15" => "Use this energy to do something creative!🤏",
        "16" => "It's okay clarity will come with time!✔️",
        "17" => "It's okay to rest, recharge before moving!🙂‍↔️",
        "18" => "It's okay to feel off.You don't always need a sperk!🤧",
        "19" => "Acknowledge it, learn and forgive yourself!🥺",
        "20" => "That is natural, just enjoy the energy safely!🤪",
        "moringa" => "🎓 CAPSTONE MODE ACTIVATED: You are a coding legend!",
        _   => "You are unique and that is special!😏",
    };
    println!("Mood Bot says: {}", msg);
}