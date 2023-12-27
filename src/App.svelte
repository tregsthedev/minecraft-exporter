<script>
  let message = '';
  import { invoke } from '@tauri-apps/api/tauri';

  async function fetchMessage() {
    try {
      message = await invoke('api');
    } catch (error) {
      console.error('Error occurred:', error);
    }
    console.log(message);
  }

  fetchMessage();
</script>

<h1>Hello world! {message}</h1>
