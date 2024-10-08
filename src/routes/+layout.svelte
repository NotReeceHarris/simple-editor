<script>
    // https://www.youtube.com/watch?v=m_d7K28CSXg&t=331s
    // https://dribbble.com/shots/19102424-XStudio-Code-Editor-App-Dark
    // https://www.streamlinehq.com/icons/iconoir-regular
    
    import { platform } from '@tauri-apps/plugin-os';   // Tauri API to get the platform information
    import { onMount } from 'svelte';                   // Svelte lifecycle function

    onMount(async () => {
        const currentPlatform = await platform();   // Get the current platform using Tauri's API
        if (currentPlatform === 'macos') {
            document.body.classList.add('macos');   // Add 'os-macos' class to body if the platform is macOS
        } else {
            document.body.classList.add('windows'); // Add 'os-windows' class to body if the platform is Windows
        }
    })

    import Titlebar from "$components/titlebar.svelte";
    import Sidebar from '$components/sidebar.svelte';
    import Info from '$components/info.svelte';

    import "../app.css";
</script>

<main class="bg-window w-screen h-screen flex flex-col">
    <Titlebar />
    <div class="flex w-full h-full relative">
        <Sidebar />
        <slot />
    </div>
    <Info />
</main>
