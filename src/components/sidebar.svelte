<script lang="ts">
    import { onMount } from "svelte";

    onMount(() => {
        const sidebar = document.getElementById("sidebar") as HTMLElement;
        const resizer = document.getElementById("resizer") as HTMLElement;
        const resizerHandle = resizer?.querySelector("#handle") as HTMLElement;

        let isResizing = false;
        let isHidden = false;

        resizer?.addEventListener("mousedown", (e) => {
            isResizing = true;
            document.body.classList.add("select-none"); // Prevents text selection while resizing
        });

        document.addEventListener("mousemove", (e) => {
            if (!isResizing) return;

            const newWidth = e.clientX; // Get the horizontal position of the cursor

            if (isHidden == false) {
                if (newWidth > 208) {
                    // Minimum width for sidebar
                    sidebar.style.width = `${newWidth}px`;
                }
                if (newWidth < 80) {
                    // Minimum width for sidebar
                    isHidden = true;
                    resizerHandle.classList.remove("hidden");
                    sidebar.style.width = `10px`;
                    sidebar.style.minWidth = `1px`;
                }
            } else {
                if (newWidth > 80) {
                    // Minimum width for sidebar
                    isHidden = false;
                    resizerHandle.classList.add("hidden");
                    sidebar.style.width = `${newWidth}px`;
                    sidebar.style.minWidth = `208px`;
                }
            }
        });

        document.addEventListener("mouseup", () => {
            isResizing = false;
            document.body.classList.remove("select-none");
        });
    });
</script>


<div class="h-[calc(100vh-72px)] border-r w-[208px] min-w-[208px] border-zinc-700 relative z-20" id="sidebar">

    <div id="resizer" class="h-full cursor-ew-resize w-6 absolute -right-3 text-white/[.5] flex place-items-center">
        <svg id="handle" class="hidden absolute right-[8px]" viewBox="-0.5 -0.5 16 16" fill="none" xmlns="http://www.w3.org/2000/svg" height="16" width="16"><path d="M7.5 7.8973125c0.219375 0 0.3973125 -0.1779375 0.3973125 -0.3973125s-0.1779375 -0.3973125 -0.3973125 -0.3973125 -0.3973125 0.1779375 -0.3973125 0.3973125 0.1779375 0.3973125 0.3973125 0.3973125Z" fill="currentColor" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="1"></path><path d="M7.5 14.2540625c0.219375 0 0.3973125 -0.1779375 0.3973125 -0.3973125s-0.1779375 -0.3973125 -0.3973125 -0.3973125 -0.3973125 0.1779375 -0.3973125 0.3973125 0.1779375 0.3973125 0.3973125 0.3973125Z" fill="currentColor" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="1"></path><path d="M7.5 1.5405625c0.219375 0 0.3973125 -0.177875 0.3973125 -0.3973125S7.719375 0.7459375 7.5 0.7459375s-0.3973125 0.177875 -0.3973125 0.3973125 0.1779375 0.3973125 0.3973125 0.3973125Z" fill="currentColor" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="1"></path></svg>
    </div>

    <div class="overflow-x-hidden relative">
        <div id="fileSystem">
            <div
                class="border-zinc-700 h-9 border-b text-white w-full place-items-center flex"
            >
                <svg
                    class="absolute left-[17px]"
                    viewBox="-0.5 -0.5 16 16"
                    fill="none"
                    xmlns="http://www.w3.org/2000/svg"
                    height="14"
                    width="14"
                    ><path
                        d="m11.26925 11.26925 3.015375 3.015375"
                        stroke="currentColor"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="1"
                    ></path><path
                        d="M0.7153750000000001 6.746124999999999c0 3.33075 2.7000625 6.0308125 6.03075 6.0308125 1.6683124999999999 0 3.1783125 -0.6773750000000001 4.270125 -1.7720625 1.0879999999999999 -1.0909375000000001 1.7606875 -2.59625 1.7606875 -4.25875 0 -3.3306875000000002 -2.7000625 -6.03075 -6.0308125 -6.03075 -3.3306875000000002 0 -6.03075 2.7000625 -6.03075 6.03075Z"
                        stroke="currentColor"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="1"
                    ></path></svg
                >
                <input
                    type="text"
                    class="w-full h-full bg-transparent placeholder-white/[.4] z-10 outline-none ps-11 pe-4 box-border text-sm"
                    placeholder="Search files..."
                />
            </div>
        </div>
    </div>

</div>