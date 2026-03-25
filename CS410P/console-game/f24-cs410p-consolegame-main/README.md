# Browser Console Text Adventure

## Description
This project is a tiny text adventure game designed to be played directly within the developer console of a web browser. Written in TypeScript, the game logic relies on browser prompts to receive player input and utilizes standard console logging functions to manage user interaction and output environment states. The objective of the game is to navigate through a series of rooms, collect necessary items, and successfully exit the building.

## Table of Contents
- [Installation](#installation)
- [Usage](#usage)
- [Features](#features)
- [Technologies Used](#technologies-used)
- [Contributing](#contributing)

## Installation 🛠️

To play this game, you need to compile the TypeScript source into JavaScript and load it into a browser environment.

1. **Clone the repository:**
   ```sh
   git clone <repository-url>
   cd <repository-directory>

```

2. **Compile the TypeScript file:**
Ensure you have TypeScript installed globally or locally, then run:
```sh
tsc Main.ts

```


3. **Load into a browser:**
Include the compiled `Main.js` file in a basic HTML document, open it in your web browser, and access the developer tools (usually F12).

## Usage 🚀

Once the script is loaded in your browser environment, you can begin the game by executing the main exported function in the console:

```javascript
play();

```

**

The game uses the built-in `prompt()` function to ask for player commands. If you are stuck, the shortest sequence of winning moves is:

1. `west`
2. `take key`
3. `east`
4. `north`
5. `open window`
6. `east`
**

## Features ✨

* **Console-Based UI:** Creatively leverages the browser's developer console for the user interface. It uses `console.info` to describe surroundings, `console.log` to echo input, `console.warn` to request input, and `console.error` to flag invalid commands.
* **State Management:** Implements simple state tracking for the player's location (`currentRoom`) and puzzle logic variables (`hasKey`, `windowOpen`).
* **Input Validation:** Safely handles empty or null inputs if a player dismisses the prompt without entering a command.

## Technologies Used 💻

* **Language:** TypeScript

## Contributing 🤝

Contributions, issues, and feature requests are welcome! Feel free to check the issues page.

