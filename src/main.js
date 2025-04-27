// main.js
// import desktopIdle from "desktop-idle";

// when using `"withGlobalTauri": true`, you may use

const { isPermissionGranted, requestPermission, sendNotification } =
  window.__TAURI__.notification;
const { ask, confirm } = window.__TAURI__.dialog;
const { listen } = window.__TAURI__.event;
const invoke = window.__TAURI__.core.invoke;

let permissionGranted;

document.addEventListener("DOMContentLoaded", () => {
  // DOM elements
  const timerDisplay = document.getElementById("timer-display");
  const statusMessage = document.getElementById("status-message");
  const startButton = document.getElementById("start-btn");
  const resetButton = document.getElementById("reset-btn");
  const settingsForm = document.getElementById("settings-form");
  const workDurationInput = document.getElementById("work-duration");
  const breakDurationInput = document.getElementById("break-duration");

  // Timer variables
  let timer;
  let isWorking = true;
  let secondsLeft = 25 * 60; // Default 25 minutes
  let workDuration = 25 * 60; // Default 25 minutes
  let breakDuration = 5 * 60; // Default 5 minutes
  let isRunning = false;
  let isPaused = false;

  // Format time as MM:SS
  function formatTime(seconds) {
    const mins = Math.floor(seconds / 60)
      .toString()
      .padStart(2, "0");
    const secs = (seconds % 60).toString().padStart(2, "0");
    return `${mins}:${secs}`;
  }

  // Update the timer display
  function updateDisplay() {
    timerDisplay.textContent = formatTime(secondsLeft);

    if (isWorking) {
      statusMessage.textContent = "Focus on your work";
      document.body.className = "working";
    } else {
      statusMessage.textContent = "Take a break!";
      document.body.className = "break";
    }
  }

  // Start/resume timer
  function startTimer() {
    if (!isRunning) {
      isRunning = true;
      startButton.textContent = "Pause";

      timer = setInterval(() => {
        secondsLeft--;
        updateDisplay();

        if (secondsLeft <= 0) {
          // Switch modes
          isWorking = !isWorking;
          secondsLeft = isWorking ? workDuration : breakDuration;

          try {
            // Create a Yes/No dialog
            ask(
              isWorking
                ? "Time to focus on your work again."
                : "Step away from your computer for a bit.",
              {
                title: isWorking ? "Break finished!" : "Time for a break!",
                kind: "warning",
              }
            ).then((ans) => {
              if (ans) {
              } else {
                resetTimer();
                startTimer();
              }
            });
          } catch (e) {
            console.error("Notification failed:", e);
          }

          // Update display after switching modes
          updateDisplay();
        }
      }, 1000);
    } else {
      clearInterval(timer);
      isRunning = false;
      startButton.textContent = "Start";
    }
  }

  // Reset timer
  function resetTimer() {
    clearInterval(timer);
    isRunning = false;
    isWorking = true;
    secondsLeft = workDuration;
    startButton.textContent = "Start";
    updateDisplay();
  }

  // Save settings
  function saveSettings(e) {
    e.preventDefault();
    workDuration = parseInt(workDurationInput.value) * 60;
    breakDuration = parseInt(breakDurationInput.value) * 60;

    // If timer is not running or we're resetting, update the current time
    if (
      !isRunning ||
      confirm(
        "Update timer with new settings? This will reset your current timer."
      )
    ) {
      resetTimer();
    }
  }

  listen("idle_state_changed", (event) => {
    if (isRunning && isWorking) {
      if (event.payload) {
        console.log("System is idle, pausing timer");
        // Add logic to pause your break timer here
        clearInterval(timer);
        isPaused = true;
      } else {
        console.log("System is active, resuming timer");
        if (isPaused) {
          isPaused = false;
          timer = setInterval(() => {
            secondsLeft--;
            updateDisplay();

            if (secondsLeft <= 0) {
              // Switch modes
              isWorking = !isWorking;
              secondsLeft = isWorking ? workDuration : breakDuration;

              // Show notification
              try {
                ask(
                  isWorking
                    ? "Time to focus on your work again."
                    : "Step away from your computer for a bit.",
                  {
                    title: isWorking ? "Break finished!" : "Time for a break!",
                    kind: "warning",
                  }
                ).then((ans) => {
                  if (ans) {
                  } else {
                    resetTimer();
                    startTimer();
                  }
                });
              } catch (e) {
                console.error("Notification failed:", e);
              }

              // Update display after switching modes
              updateDisplay();
            }
          }, 1000);
        }
      }
    }
  });

  // Initialize app
  async function init() {
    // Set default values in form
    workDurationInput.value = Math.floor(workDuration / 60);
    breakDurationInput.value = Math.floor(breakDuration / 60);

    // Add event listeners
    startButton.addEventListener("click", startTimer);
    resetButton.addEventListener("click", resetTimer);
    settingsForm.addEventListener("submit", saveSettings);

    // Do you have permission to send a notification?
    // permissionGranted = await isPermissionGranted();

    // If not we need to request it
    // if (!permissionGranted) {
    //   const permission = await requestPermission();
    //   permissionGranted = permission === "granted";
    //   sendNotification({
    //     title: "Test Notification",
    //     body: "Notification is working ",
    //   });
    // }

    // Initial display update
    updateDisplay();
  }

  // Initialize when DOM is loaded
  init();
});
