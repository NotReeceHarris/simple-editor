<script>
    // https://www.youtube.com/watch?v=m_d7K28CSXg&t=331s
    // https://dribbble.com/shots/19102424-XStudio-Code-Editor-App-Dark
    // https://www.streamlinehq.com/icons/iconoir-regular
    
    import Nav from "$components/nav.svelte";
    import Sidebar from "$components/sidebar.svelte";
    import Info from "$components/info.svelte";

    import "../app.css";
</script>

<main class="bg-window w-screen h-screen">
    <Nav />
    <div class="flex">
        <Sidebar />
        <div class="bg-editor flex-grow text-white flex flex-col h-[calc(100vh-72px)]">
            <div class="flex grow">
                <slot />
            </div>
            <Info />
        </div>
    </div>
</main>
