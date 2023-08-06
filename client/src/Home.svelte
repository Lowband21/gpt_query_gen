<script>
  import { onMount, onDestroy } from "svelte";
  import { fly } from "svelte/transition";
  import { flip } from "svelte/animate";
  import { Link } from "svelte-routing";


  onMount(async () => {
  });
</script>

<div class="homepage">
  <!-- Welcome Section -->
  <div class="full-width-container welcome-container">
    <div class="section-content">
      <h1>Welcome to GPT Query Generator!</h1>
    </div>
  </div>
</div>

<style>
  .homepage {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 0em;
    min-height: 100vh;
    transition: all 0.1s ease-in-out;
    position: relative;
    z-index: 1;
  }

  .text-block {
    background-color: #333333;
    border: 3px solid #000000;
    padding: 20px; /* Add some padding so that the background extends beyond just the text */
    border-radius: 20px;
    opacity: 90%;
  }

  h1 {
    font-size: 3em;
    color: #000000;
  }

  h2 {
    font-size: 3em;
    margin-bottom: 1em auto;
    color: #ffffff;
    text-justify: center;
    justify-content: center;
    justify-self: center;
  }

  h3 {
    font-size: 2em;
    margin-bottom: 1em;
    color: #ffffff;
  }

  p {
    font-size: 1.5em;
    color: #ffffff;
  }

  .full-width-container {
    width: 100vw;
    display: flex;
    justify-content: center;
  }

  .welcome-container {
    text-align: center;
    padding: 2em 0;
  }

  .section-content {
    width: 80%;
  }
  
</style>

