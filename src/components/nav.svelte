<script lang="ts">
    import { onMount } from "svelte";
    import { getCurrentWindow } from "@tauri-apps/api/window";

    onMount(() => {
        const appWindow = getCurrentWindow();

        document
            .getElementById("titlebar-minimize")
            ?.addEventListener("click", () => appWindow.minimize());
        document
            .getElementById("titlebar-maximize")
            ?.addEventListener("click", () => appWindow.toggleMaximize());
        document
            .getElementById("titlebar-close")
            ?.addEventListener("click", () => appWindow.close());
    });
</script>

<nav
    data-tauri-drag-region
    class="select-none flex gap-2 p-3 justify-start top-0 right-0 left-0 text-white border-b border-zinc-700"
>

<div class="cursor-pointer bg-red-400 w-3 h-3 rounded-full" id="titlebar-close"></div>
<div class="cursor-pointer bg-amber-300 w-3 h-3 rounded-full" id="titlebar-maximize"></div>
<div class="cursor-pointer bg-green-400 w-3 h-3 rounded-full" id="titlebar-minimize"></div>

</nav>
