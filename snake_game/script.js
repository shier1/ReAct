// Game variables
const canvas = document.getElementById('gameCanvas');
const ctx = canvas.getContext('2d');
const scoreElement = document.getElementById('score');
const startBtn = document.getElementById('startBtn');
const pauseBtn = document.getElementById('pauseBtn');
const resetBtn = document.getElementById('resetBtn');

const gridSize = 20;
const tileCount = canvas.width / gridSize;

let snake = [
    {x: 10, y: 10}
];
let food = {};
let dx = 0;
let dy = 0;
let score = 0;
let gameRunning = false;
let gamePaused = false;
let gameLoop;

// Initialize food
generateFood();

// Event listeners for buttons
startBtn.addEventListener('click', startGame);
pauseBtn.addEventListener('click', togglePause);
resetBtn.addEventListener('click', resetGame);

// Keyboard controls
document.addEventListener('keydown', changeDirection);

// Generate random food position
function generateFood() {
    food = {
        x: Math.floor(Math.random() * tileCount),
        y: Math.floor(Math.random() * tileCount)
    };
    
    // Make sure food doesn't appear on snake
    for (let segment of snake) {
        if (segment.x === food.x && segment.y === food.y) {
            generateFood();
            return;
        }
    }
}

// Draw game elements
function draw() {
    // Clear canvas
    ctx.fillStyle = 'black';
    ctx.fillRect(0, 0, canvas.width, canvas.height);
    
    // Draw snake
    ctx.fillStyle = 'lime';
    for (let segment of snake) {
        ctx.fillRect(segment.x * gridSize, segment.y * gridSize, gridSize - 2, gridSize - 2);
    }
    
    // Draw food
    ctx.fillStyle = 'red';
    ctx.fillRect(food.x * gridSize, food.y * gridSize, gridSize - 2, gridSize - 2);
}

// Update game state
function update() {
    if (!gameRunning || gamePaused) return;
    
    // Move snake head
    const head = {x: snake[0].x + dx, y: snake[0].y + dy};
    
    // Check wall collision
    if (head.x < 0 || head.x >= tileCount || head.y < 0 || head.y >= tileCount) {
        gameOver();
        return;
    }
    
    // Check self collision
    for (let segment of snake) {
        if (head.x === segment.x && head.y === segment.y) {
            gameOver();
            return;
        }
    }
    
    // Add new head
    snake.unshift(head);
    
    // Check food collision
    if (head.x === food.x && head.y === food.y) {
        // Increase score
        score += 10;
        scoreElement.textContent = score;
        
        // Generate new food
        generateFood();
    } else {
        // Remove tail if no food eaten
        snake.pop();
    }
}

// Game loop
function gameStep() {
    update();
    draw();
}

// Start game
function startGame() {
    if (gameRunning && !gamePaused) return;
    
    if (!gameRunning) {
        // Reset snake and direction if starting fresh
        snake = [{x: 10, y: 10}];
        dx = 0;
        dy = 0;
        score = 0;
        scoreElement.textContent = score;
        generateFood();
    }
    
    gameRunning = true;
    gamePaused = false;
    pauseBtn.textContent = 'Pause';
    
    if (gameLoop) clearInterval(gameLoop);
    gameLoop = setInterval(gameStep, 100);
}

// Toggle pause
function togglePause() {
    if (!gameRunning) return;
    
    gamePaused = !gamePaused;
    pauseBtn.textContent = gamePaused ? 'Resume' : 'Pause';
}

// Reset game
function resetGame() {
    gameRunning = false;
    gamePaused = false;
    snake = [{x: 10, y: 10}];
    dx = 0;
    dy = 0;
    score = 0;
    scoreElement.textContent = score;
    generateFood();
    pauseBtn.textContent = 'Pause';
    
    if (gameLoop) {
        clearInterval(gameLoop);
        gameLoop = null;
    }
    
    draw();
}

// Game over
function gameOver() {
    gameRunning = false;
    if (gameLoop) {
        clearInterval(gameLoop);
        gameLoop = null;
    }
    alert('Game Over! Your score: ' + score);
}

// Change direction based on key press
function changeDirection(event) {
    if (!gameRunning || gamePaused) return;
    
    const key = event.keyCode;
    const goingUp = dy === -1;
    const goingDown = dy === 1;
    const goingRight = dx === 1;
    const goingLeft = dx === -1;
    
    // Left arrow or A
    if ((key === 37 || key === 65) && !goingRight) {
        dx = -1;
        dy = 0;
    }
    // Up arrow or W
    if ((key === 38 || key === 87) && !goingDown) {
        dx = 0;
        dy = -1;
    }
    // Right arrow or D
    if ((key === 39 || key === 68) && !goingLeft) {
        dx = 1;
        dy = 0;
    }
    // Down arrow or S
    if ((key === 40 || key === 83) && !goingUp) {
        dx = 0;
        dy = 1;
    }
}

// Initial draw
draw();