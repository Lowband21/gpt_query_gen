<script>
  import { onMount, onDestroy } from "svelte";
  import { Router, Route, Link } from 'svelte-routing';
  import Home from './Home.svelte';
  import Chat from './Chat.svelte';

  import Particles from "svelte-particles";

  let particlesConfig = {
    fpsLimit: 120,
    particles: {
      color: {
        value: "#000"
      },
      links: {
        enable: true,
        color: "#000"
      },
      move: {
        enable: true
      }
    }
  };

</script>

<style>
.background {
  height: 100%;
  width: 100%;
  margin: 0;
  padding: 0;
}
html, body {
  margin: 0;
  padding: 0;
  width: 100%;
  height: 100%;
}
.navbar-link {
  display: inline-block;
  padding: 10px 20px;
  margin: 10px;
  background-color: #5294E2;  /* Changed to Bootstrap's primary color */
  color: #ffffff !important;             /* Changed to white */
  text-decoration: none;
  border-radius: 5px;
  transition: background-color 0.3s ease-in-out;
}

.navbar-link:hover {
  background-color: #0056b3;  /* Darkened version of primary color */
}

.navbar {
  display: flex;
  justify-content:space-around;
  padding: 10px 0;
  width: 100%;
  height:max-content;
  background-color: #333333;  /* Changed to Bootstrap's light color */
  border-radius: 0px; /* New property */
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  margin: 0;
  padding: 0;
  z-index: 9999;
}

.navbar-link :global(a){
  color: #000000;  /* Changed to white */
  text-decoration: none;
  display: block;
  width: 100%;
  height: 100%;
}
.navbar-link :global(a:hover), .navbar-link :global(a:focus) {
  color: #000000;
}
:global(#tsparticles) {
  margin: 0;
  padding: 0;
  position: fixed;
  width: 100%;
  height: 100%;
  top: 0;
  left: 0;
  z-index: -9999;
}
</style>



<Particles options={particlesConfig} />
<div class="background">
<Router>
  <nav class="navbar">
    <div class="navbar-link"><Link to="/">Home</Link></div>
    <div class="navbar-link"><Link to="/chat">Chat</Link></div>
  </nav>
  <Route path="/" component={Home} />
  <Route path="/chat" component={Chat} />
</Router>
</div>