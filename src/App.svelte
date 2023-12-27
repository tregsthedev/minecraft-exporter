<script>
  let message = '';
  import { invoke } from '@tauri-apps/api/tauri';

  async function fetchMessage() {
    try {
      message = await invoke('api');
    } catch (error) {
      console.error('Error occurred:', error);
    }
  }

  fetchMessage();
</script>

<h1>Hello world! {message}</h1>
