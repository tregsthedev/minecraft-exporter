<script>
  let message = '';
  let playerCount = 2;
  let data = '';
  let serverName = ''
  let minPlayerCount = 0
  let motd = '';

  import { invoke } from '@tauri-apps/api/tauri';

  async function fetchMessage() {
    try {
      message = await invoke('api')
      data = JSON.parse(message)
      // @ts-ignore
      serverName = data.version.name
      // @ts-ignore
      let {version, players } = data
      // @ts-ignore
      minPlayerCount = data.players.max
      // @ts-ignore
      playerCount = data.players.online
      // @ts-ignore
      motd = data.motd.text



    } catch (error) {
      console.error('Error occurred:', error);
    }
    console.log(message);
  }

  fetchMessage();
</script>

<div class="header">
<center> <h1> Minecraft Server Stats </h1> </center>
<h1 class="title"> Server Name: {serverName} </h1>
<h4> {motd} </h4>
<center> <input>  </center>

</div>


<h3> <span class="server-name"> Server Stats </span> </h3>
<p>

<span class="player-count">Max player count: </span> {playerCount}
<br>
<span>Total player count: {minPlayerCount} </span>
</p>


{#if name == " Purpur 1.20.1"}
	<p>{count} is greater than 10</p>
{/if}
<style>
h1 {
color: blue;
}
</style>