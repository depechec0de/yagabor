<script lang="ts">
  import { onMount } from 'svelte'

  import initWasm, { EmulationWasm, type InitOutput } from "$lib/wasm/gameboy";

  import { 
    SCREEN_WIDTH, 
    SCREEN_HEIGHT,
    MS_BETWEEN_FRAMES } from "$lib/constants";

  import KeyPad from "./KeyPad.svelte";
  import Screen from "./Screen.svelte";

  let emu : EmulationWasm | undefined;
  let wasmInstance : InitOutput | undefined;
  let screenPtr : number | undefined;
  let powerstatus : boolean;
  let screenbuffer : Uint8Array = new Uint8Array(SCREEN_WIDTH * SCREEN_HEIGHT);
  let animationFrame : number;
  let lastTimestamp : number;
  
  function togglepower() {
		powerstatus = !powerstatus;
	}

  onMount(() => {
    initWasm().then((instance) => {
      wasmInstance = instance;

      emu = EmulationWasm.new();
    }); 

    return () => cancelAnimationFrame(animationFrame)
  });

  function step(timestamp : number){
    screenPtr = emu?.screen();
    //console.log(screenPtr)
    if(wasmInstance != null){
      screenbuffer = new Uint8Array(wasmInstance.memory.buffer, screenPtr, SCREEN_WIDTH * SCREEN_HEIGHT);
      //console.log(`${screenbuffer}`)
    }
  
    if(powerstatus){
      if (lastTimestamp == null) {
        lastTimestamp = timestamp;
      }
      const elapsed = timestamp - lastTimestamp;
      
      if (elapsed > MS_BETWEEN_FRAMES) {
        
        let cycles = emu?.step();
        //console.log(cycles)
        lastTimestamp = timestamp;
      } 

      animationFrame = requestAnimationFrame(step);
    }else{
      cancelAnimationFrame(animationFrame);
    }
  }
  
  $: if(powerstatus){
    animationFrame = requestAnimationFrame(step);
  }else{
    if(emu != null){
      emu = EmulationWasm.new();
    }
  }

  //$: console.log(screenbuffer)

</script>

<div class="gameboy {powerstatus? 'power-on' : ''}">

  <div class="front-plate">
     <div class="front-plate-head">
        <div class="vertical-stripe"></div>
        <div class="vertical-stripe"></div>
        <div class="vertical-stripe"></div>
        <div class="vertical-gouge vertical-gouge-1"></div>
        <div class="vertical-gouge vertical-gouge-2"></div>
        <div class="on-off">
           <div class="spike spike-left">
              <div></div>
           </div>
           <div class="spike spike-right">
              <div></div>
           </div>
           <span>
           OFF
           <i></i>
           ON
           </span>
        </div>
     </div>

     <Screen powerstatus={powerstatus} screenbuffer={screenbuffer}/>

     <div class="logo"></div>

     <KeyPad/>

     <div class="speaker">
        <div>
           <div class="speaker-inner-shadow"></div>
        </div>
        <div>
           <div class="speaker-inner-shadow"></div>
        </div>
        <div>
           <div class="speaker-inner-shadow"></div>
        </div>
        <div>
           <div class="speaker-inner-shadow"></div>
        </div>
        <div>
           <div class="speaker-inner-shadow"></div>
        </div>
        <div>
           <div class="speaker-inner-shadow"></div>
        </div>
     </div>
     <div class="phones" id="volume-switch">
        <div class="vertical-stripe"></div>
        <div class="vertical-stripe"></div>
        <div class="vertical-stripe"></div>
        <i></i>
        <span>PHONES</span>
     </div>
  </div>
  <div on:click={togglepower} class="power-button">
     <div></div>
  </div>
</div>

<style>
@import './gameboy.css';
</style>