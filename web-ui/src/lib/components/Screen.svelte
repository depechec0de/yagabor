<script lang="ts">
  import { onMount } from 'svelte'

  import { ColoredPixel } from "$lib/wasm/gameboy";
  import { SCREEN_WIDTH, SCREEN_HEIGHT } from "$lib/constants";

  export let screenbuffer : Uint8Array;
  export let powerstatus : boolean;
  
  // Canvas element size
  let canvasWidth : number = 352
  let canvasHeight : number = 316
  // Size of a screen pixel in the canvas
  let pixelWidth : number = canvasWidth / SCREEN_WIDTH;
  let pixelHeight : number = canvasHeight / SCREEN_HEIGHT;

  let canvas : HTMLCanvasElement
  let canvasContext : CanvasRenderingContext2D | null = null;

  function drawPixel(x: number, y: number, pixelcolor: ColoredPixel){
    let color;

    switch (pixelcolor) {
      case ColoredPixel.Black:
        color = "#081820";
        break;
      case ColoredPixel.DarkGray:
        color = "#346856";
        break;
      case ColoredPixel.LightGray:
        color = "#88c070";
        break;
      case ColoredPixel.White:
        color = "#e0f8d0";
        break;
    }
    if(canvasContext != null){
      canvasContext.fillStyle = color;
    }
    canvasContext?.fillRect(x*pixelWidth, y*pixelHeight, pixelWidth, pixelHeight);
  }

  function render(screen: Uint8Array | undefined){
    if(screen == null){
      return
    }

    canvasContext?.clearRect(0, 0, canvas.width, canvas.height);
 
    for (let x = 0; x < SCREEN_WIDTH; x++) {
      for (let y = 0; y < SCREEN_HEIGHT; y++) {
        drawPixel(x, y, screen[x + SCREEN_WIDTH * y])
      }
    }
  }

  onMount(() => {
    canvasContext = canvas.getContext('2d');
	})

  $: if(canvasContext != null){
    render(screenbuffer)
  }

  $: if(!powerstatus){
    canvasContext?.clearRect(0, 0, canvas.width, canvas.height);
  }
</script>

<div class="screen-container">
  <div class="screen-headline">
     <span>DOT MATRIX WITH STEREO SOUND</span>
  </div>
  <div class="battery-light">
     <span>BATTERY</span>
  </div>
  <div class="screen">
    <canvas 
      width={canvasWidth}
      height={canvasHeight}
      bind:this={canvas}
    />
  </div>
</div>

<style>
@import './screen.css';
</style>