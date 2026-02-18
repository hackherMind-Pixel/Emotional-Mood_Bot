# Emotional-Mood_Bot
I used 'Rust' technology : To create a functional command-line interface (CLI) that processes user strings and integers to deliver specific "mood reports."

# 🤖 Mood Bot: A Rust-Powered Emotional Companion

A command-line interface (CLI) tool built with **Rust** that interacts with users to provide personalized, supportive feedback based on their current emotional state. This project was developed as a **Moringa AI Capstone Project** to explore systems programming and generative AI integration.

## ✨ Features
* **User Personalization:** Greets users by name for an empathetic experience.
* **20 Unique Mood Responses:** Uses a comprehensive `match` statement to provide varied emotional support for states ranging from 'Happy' to 'Frisky'.
* **Input Sanitization:** Uses `.trim()` to handle trailing newlines and ensure accurate matching.
* **Hidden Easter Egg:** Includes a secret "Moringa" command for a special capstone message.

## 🛠️ Built With
* **Language:** Rust 🦀
* **Build System:** Cargo
* **IDE:** VS Code (Selected after iterating away from Notepad to resolve formatting errors)

## 🚀 Getting Started

### Prerequisites
* You must have **Rust** and **Cargo** installed. Get them at [rustup.rs](https://rustup.rs/).

### Installation
1. **Clone the repository:**
   ```bash
   git clone https://github.com/hackherMind-Pixel/Emotional-Mood_Bot.git
   cd rust-mood-bot
2.Run the application:
command:cargo run
🖥️ Usage Example
When you run the bot, it will prompt you for your name and a number corresponding to your mood:

Plaintext
=== Welcome to Mood Bot! ===
What is your name?
rnb
Hey rnb! How are you feeling?
1. Happy
...
9. Agitated
...
9
Mood Bot says: its okay to feel agitated, this moment will settle! 🤞
🧠 Lessons Learned (Iteration & Testing)
This project underwent a significant transformation during development:

The Challenge: Initially using Notepad caused the code to compress into a single line, leading to "unclosed delimiter" errors.

The Solution: Migrated the development environment to VS Code, which provided the necessary syntax highlighting and bracket matching to fix logic errors.
