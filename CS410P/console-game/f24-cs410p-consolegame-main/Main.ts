//Matthew Penner

// A Room value is exactly one of these four strings.
// It is impossible for a Room variable to contain any other string.
type Room = "A" | "B" | "C" | "Exit";
let currentRoom: Room = "A";
let hasKey: boolean = false;
let windowOpen: boolean = false;
function handleUnrecognizedCommand() {
  console.error("Unrecognized command.");
}
function goToRoomA(): void {
  currentRoom = "A";
  console.info("You are in an empty room. There are doors on the north and west walls of this room.");
}
function goToRoomB() {
  currentRoom = "B";
  console.info("You go through the west door. You are in a room with a table.");
  if (!hasKey) {
    console.info("On the table there is a key.");
  }
  console.info("There is a door on the east wall of this room.");
}
function goToRoomC() {
  currentRoom = "C";
  console.info("You unlock the north door with the key and go through the door.");
  console.info("You are in a bright room. There is a door on the south wall of this room and a window on the east wall.");   
}
function takeKey() {
  if (hasKey) {
    console.error("You already have the key.");
  } else {
    console.info("You take the key from the table.");
    hasKey = true;
  }
}
function openWindow() {
  if (windowOpen) {
    console.error("The window is already open.");
  } else {
    console.info("You open the window.");
    windowOpen = true;
  }
}
function tryOpenNorthDoor() {
  if (hasKey) {
    goToRoomC();
  } else {
    console.error("You try to open the north door, but it is locked.");
  }
}
function tryGoEastFromRoomC() {
  if (windowOpen) {
    currentRoom = "Exit";
    console.info("You step out from the open window.");
  } else {
    console.error("The window is closed.");
  }
}
function inputFunction(message: string): string {
  let input: string | null = prompt(message);
  while (input == null || input == "") {
    console.error("Invalid input.");
    input = prompt("Please enter your name.");
  }
  console.log(input)
  return input;
}
export function play(): void {
  console.info("Welcome to the text adventure! Open your browser's developer console to play.");
  console.warn("Please enter your name.");
  let playerName: string = inputFunction("Please enter your name.");
  console.info("Hello, " + playerName + ".");
  console.info("You are in a building. Your goal is to exit this building.");
  console.info("You are in an empty room. There are doors on the north and west walls of this room.");

  while (currentRoom != "Exit") {
    console.warn("Please enter a command.");
    let command: string = inputFunction("Please enter a command.");

    switch (currentRoom) {
      case "A":
        switch (command) {
          case "west":
            goToRoomB();
            break;
          case "north":
            tryOpenNorthDoor();
            break;
          default:
            handleUnrecognizedCommand();
        }
        break;
      case "B":
        switch (command) {
          case "east":
            goToRoomA();
            break;
          case "take key":
            takeKey();
            break;
          default:
            handleUnrecognizedCommand();
        }
        break;
      case "C":
        switch (command) {
          case "south":
            goToRoomA();
            break;
          case "east":
            tryGoEastFromRoomC()
            break;
          case "open window":
            openWindow();
            break;
          default:
            handleUnrecognizedCommand();
        }
        break;
    }
  }
  console.info("You have exited the building. You win!");
  console.info("Congratulations, " + playerName + "!");
}
