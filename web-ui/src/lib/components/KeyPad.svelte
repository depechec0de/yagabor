<script lang="ts">
  let topPressed = false;
  let downPressed = false;
  let leftPressed = false;
  let rightPressed = false;
  let aPressed = false;
  let bPressed = false;
  let selectPressed = false;
  let startPressed = false;

  function handleKeydown(event : KeyboardEvent){
    switch (event.key) {
      case "ArrowUp":
        topPressed = true;
        break;
      case "ArrowDown":
        downPressed = true;
        break;
      case "ArrowLeft":
        leftPressed = true;
        break;
      case "ArrowRight":
        rightPressed = true;
        break;
      case "a":
        aPressed = true;
        break;
      case "s":
        bPressed = true;
        break;
      case "Backspace":
        selectPressed = true;
        break;
      case "Enter":
        startPressed = true;
        break;
    }
  }

  function handleKeyup(event : KeyboardEvent){
    switch (event.key) {
      case "ArrowUp":
        topPressed = false;
        break;
      case "ArrowDown":
        downPressed = false;
        break;
      case "ArrowLeft":
        leftPressed = false;
        break;
      case "ArrowRight":
        rightPressed = false;
        break;
      case "a":
        aPressed = false;
        break;
      case "s":
        bPressed = false;
        break;
      case "Backspace":
        selectPressed = false;
        break;
      case "Enter":
        startPressed = false;
        break;
    }
  }
</script>
<svelte:window on:keydown={handleKeydown} on:keyup={handleKeyup} />
<div id="controller">
  <div class="buttons-a-b">
     <div class="button-b button-key-j {aPressed? 'btnPressed' : ''}" id="controller_b"></div>
     <div class="button-a button-key-k {bPressed? 'btnPressed' : ''}" id="controller_a"></div>
  </div>
  <div class="start button-key-m {startPressed? 'btnPressed' : ''}" id="controller_start">
     <div></div>
  </div>
  <div class="select button-key-n {selectPressed? 'btnPressed' : ''}" id="controller_select">
     <div></div>
  </div>
  <div class="cross-container">
     <div class="spike">
        <div></div>
     </div>
     <div class="spike">
        <div></div>
     </div>
     <div class="spike">
        <div></div>
     </div>
     <div class="spike">
        <div></div>
     </div>
     <div class="cross" id="controller_dpad">
        <div class="top-down">
           <div class="button-top button-key-w {topPressed? 'btnPressed' : ''}" id="controller_up">
              <div class="button-stripe"></div>
              <div class="button-stripe"></div>
              <div class="button-stripe"></div>
           </div>
           <div class="button-bottom button-key-s {downPressed? 'btnPressed' : ''}" id="controller_down">
              <div class="button-stripe"></div>
              <div class="button-stripe"></div>
              <div class="button-stripe"></div>
           </div>
        </div>
        <div class="left-right">
           <div class="button-left button-key-a {leftPressed? 'btnPressed' : ''}" id="controller_left">
              <div class="button-stripe"></div>
              <div class="button-stripe"></div>
              <div class="button-stripe"></div>
           </div>
           <div class="button-right button-key-d {rightPressed? 'btnPressed' : ''}" id="controller_right">
              <div class="button-stripe"></div>
              <div class="button-stripe"></div>
              <div class="button-stripe"></div>
           </div>
        </div>
        <div class="cross-middle-bumb"></div>
     </div>
  </div>
</div>

<style>
@import './keypad.css';
</style>