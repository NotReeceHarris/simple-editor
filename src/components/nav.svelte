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

<nav class="select-none flex flex-col z-50 w-screen">
    <div
        data-tauri-drag-region
        class="fixed w-screen h-9 max-h-9 border-zinc-700 border-b"
    ></div>

    <div class="flex text-white w-screen">
        <div
            class="flex gap-2 w-52 min-w-52 h-9 px-3 place-items-center ps-4 pe-4 border-r border-zinc-700"
        >
            <div
                class="cursor-pointer bg-red-400 w-3 h-3 rounded-full z-10"
                id="titlebar-close"
            ></div>
            <div
                class="cursor-pointer bg-amber-300 w-3 h-3 rounded-full z-10"
                id="titlebar-maximize"
            ></div>
            <div
                class="cursor-pointer bg-green-400 w-3 h-3 rounded-full z-10"
                id="titlebar-minimize"
            ></div>
            <div class="flex-grow"></div>
            <svg
                class="cursor-pointer z-10"
                viewBox="-0.5 -0.5 16 16"
                fill="none"
                xmlns="http://www.w3.org/2000/svg"
                height="14"
                width="14"
                ><path
                    d="M2.579 0.889375c-0.35424999999999995 -0.2099375 -0.8023750000000001 0.0454375 -0.8023750000000001 0.4571875v12.3068125c0 0.4118125 0.448125 0.6671874999999999 0.8023750000000001 0.45725000000000005l10.3839375 -6.153375c0.34731249999999997 -0.20587500000000003 0.34731249999999997 -0.708625 0 -0.9145000000000001L2.579 0.889375Z"
                    stroke="currentColor"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="1"
                ></path></svg
            >
        </div>
        <div class="flex place-items-center flex-grow px-4">
            <div class="flex place-items-center flex-grow text-sm">
                simple-editor-backend
            </div>
            <div class="flex place-items-center gap-3 text-white/[.6] z-10">

                <div class="cursor-pointer">
                    <svg
                        viewBox="-0.5 -0.5 16 16"
                        fill="none"
                        xmlns="http://www.w3.org/2000/svg"
                        height="16"
                        width="16"
                        ><path
                            d="M7.5 9.551187500000001c1.132875 0 2.0511874999999997 -0.9183125000000001 2.0511874999999997 -2.0511874999999997S8.632875 5.4488125 7.5 5.4488125 5.4488125 6.367125 5.4488125 7.5 6.367125 9.551187500000001 7.5 9.551187500000001Z"
                            stroke="currentColor"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="1"
                        ></path><path
                            d="M12.7115625 6.402875 11.961062499999999 4.590625l1.0086875 -1.1929375 -1.3674374999999999 -1.3674374999999999 -1.1864375 1.013875 -1.85075 -0.761125 -0.42562500000000003 -1.6201875h-1.3361875l-0.4320625 1.6416875 -1.8082500000000001 0.76225 -1.1653125 -1.0365 -1.3674374999999999 1.3674374999999999 0.9936875000000001 1.2230625000000002 -0.7389999999999999 1.8169374999999999 -1.622125 0.3785625v1.3675000000000002l1.6416875 0.448125 0.7620625000000001 1.8079375 -1.0363125 1.1625 1.3674374999999999 1.3674374999999999 1.224625 -0.998 1.7816874999999999 0.7329375 0.41225 1.6325h1.3675000000000002l0.41325 -1.6318750000000002 1.8123125 -0.7505000000000001c0.3020625 0.21587499999999998 1.193 1.0149374999999998 1.193 1.0149374999999998l1.3674374999999999 -1.3674374999999999 -1.0146875 -1.1969375 0.7506875000000001 -1.8128125000000002 1.6313749999999998 -0.4244375 0.0000625 -1.351875 -1.6255625 -0.413375Z"
                            stroke="currentColor"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="1"
                        ></path></svg
                    >
                </div>

                <div class="bg-gray-500/[.4] w-[1px] h-4"></div>
            
                <div class="cursor-pointer">
                    <svg viewBox="-0.5 -0.5 16 16" fill="none" xmlns="http://www.w3.org/2000/svg" height="16" width="16"><path d="M7.5 14.337187499999999c3.7760624999999997 0 6.837187500000001 -3.061125 6.837187500000001 -6.837187500000001 0 -3.7760624999999997 -3.061125 -6.837187500000001 -6.837187500000001 -6.837187500000001C3.7239375000000003 0.6628125 0.6628125 3.7239375000000003 0.6628125 7.5c0 3.7760624999999997 3.061125 6.837187500000001 6.837187500000001 6.837187500000001Z" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="1"></path><path d="M5.4488125 5.4488125c0 -2.3930000000000002 3.7605 -2.3930000000000002 3.7605 0 0 1.7093125 -1.7093125 1.367375 -1.7093125 3.4185624999999997" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="1"></path><path d="m7.5 11.6091875 0.0068125 -0.007625000000000001" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="1"></path></svg>
                </div>
            
            </div>
        </div>
    </div>
    

    <div class="border-zinc-700 h-9 max-h-9 top-9 border-b text-white w-full z-10 bg-window">
        <div
            class="flex justify-evenly place-items-center w-52 h-9 border-r border-zinc-700"
        >
            <div class="cursor-pointer">
                <svg
                    viewBox="-0.5 -0.5 16 16"
                    fill="none"
                    xmlns="http://www.w3.org/2000/svg"
                    height="16"
                    width="16"
                    ><path
                        d="M0.6628125 6.81625V2.4404375c0 -0.2265625 0.183625 -0.4101875 0.4101875 -0.4101875h4.2240625c0.09793750000000001 0 0.192625 0.035 0.267 0.09875l2.1625625 1.853625c0.074375 0.06375 0.1690625 0.09875 0.267 0.09875h5.933375c0.2265625 0 0.4101875 0.1836875 0.4101875 0.41025v2.3246249999999997m-13.674375000000001 0v5.7433125c0 0.2265625 0.183625 0.4101875 0.4101875 0.4101875h12.854000000000001c0.2265625 0 0.4101875 -0.183625 0.4101875 -0.4101875V6.81625m-13.674375000000001 0h13.674375000000001"
                        stroke="currentColor"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="1"
                    ></path></svg
                >
            </div>

            <div class="cursor-pointer">
                <svg
                    viewBox="-0.5 -0.5 16 16"
                    fill="none"
                    xmlns="http://www.w3.org/2000/svg"
                    height="16"
                    width="16"
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
            </div>

            <div class="cursor-pointer">
                <svg
                    viewBox="-0.5 -0.5 16 16"
                    fill="none"
                    xmlns="http://www.w3.org/2000/svg"
                    height="16"
                    width="16"
                    ><path
                        d="M11.26925 3.73075c0.8326875 0 1.5076875 -0.675 1.5076875 -1.5076875s-0.675 -1.5076875 -1.5076875 -1.5076875 -1.5076875 0.675 -1.5076875 1.5076875 0.675 1.5076875 1.5076875 1.5076875Z"
                        stroke="currentColor"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="1"
                    ></path><path
                        d="M3.73075 3.73075c0.8326875 0 1.5076875 -0.675 1.5076875 -1.5076875s-0.675 -1.5076875 -1.5076875 -1.5076875 -1.5076875 0.675 -1.5076875 1.5076875S2.898125 3.73075 3.73075 3.73075Z"
                        stroke="currentColor"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="1"
                    ></path><path
                        d="M3.73075 14.284625c0.8326875 0 1.5076875 -0.675 1.5076875 -1.5076875s-0.675 -1.5076875 -1.5076875 -1.5076875 -1.5076875 0.675 -1.5076875 1.5076875 0.6750625 1.5076875 1.5076875 1.5076875Z"
                        stroke="currentColor"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="1"
                    ></path><path
                        d="M3.73075 3.73075v7.5385"
                        stroke="currentColor"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="1"
                    ></path><path
                        d="M11.26925 3.73075v0.753875c0 1.884625 -1.5076875 2.2615 -1.5076875 2.2615l-4.523125 1.50775s-1.5076875 0.37687499999999996 -1.5076875 2.2615v0.753875"
                        stroke="currentColor"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="1"
                    ></path></svg
                >
            </div>

            <div class="cursor-pointer">
                <svg
                    viewBox="-0.5 -0.5 16 16"
                    fill="none"
                    xmlns="http://www.w3.org/2000/svg"
                    height="16"
                    width="16"
                    ><path
                        d="M8.867437500000001 14.139C8.4341875 14.2679375 7.9751875000000005 14.337187499999999 7.5 14.337187499999999C4.85675 14.337187499999999 2.7139375 12.1944375 2.7139375 9.551187500000001V7.45725C2.7139375 5.9704375 3.9192500000000003 4.765125 5.406125 4.765125H9.593874999999999C11.0806875 4.765125 12.2860625 5.9704375 12.2860625 7.45725V9.551187500000001C12.2860625 10.855250000000002 11.7645 12.0374375 10.918624999999999 12.9006875"
                        stroke="currentColor"
                        stroke-linecap="round"
                        stroke-width="1"
                    ></path><path
                        d="M4.9360625 4.42325C4.9360625 4.1400625 4.7064375 3.9104375000000005 4.42325 3.9104375000000005S3.9104375000000005 4.1400625 3.9104375000000005 4.42325H4.9360625ZM4.51525 2.4286875C4.3576875 2.6640625 4.42075 2.9825625000000002 4.656124999999999 3.1400625C4.8914375 3.297625 5.2099375000000006 3.2345625 5.3675 2.9991875L4.51525 2.4286875ZM11.089562500000001 5.107V4.42325H10.0639375V5.107H11.089562500000001ZM3.9104375000000005 4.42325V5.107H4.9360625V4.42325H3.9104375000000005ZM11.089562500000001 4.42325C11.089562500000001 2.4408125 9.4824375 0.8336875 7.5 0.8336875V1.8593125C8.9160625 1.8593125 10.0639375 3.0071874999999997 10.0639375 4.42325H11.089562500000001ZM7.5 0.83375C6.2549375 0.83375 5.158312499999999 1.4680000000000002 4.51525 2.4286875L5.3675 2.9991875C5.8283125 2.31075 6.6115 1.8593125 7.5 1.8593125V0.83375Z"
                        fill="currentColor"
                        stroke-width="1"
                    ></path><path
                        d="M12.2860625 8.867437500000001H14.337187499999999"
                        stroke="currentColor"
                        stroke-linecap="round"
                        stroke-width="1"
                    ></path><path
                        d="M2.7139375 8.867437500000001H0.6628125"
                        stroke="currentColor"
                        stroke-linecap="round"
                        stroke-width="1"
                    ></path><path
                        d="M9.2093125 1.688375L10.918624999999999 0.6628125"
                        stroke="currentColor"
                        stroke-linecap="round"
                        stroke-width="1"
                    ></path><path
                        d="M5.790687500000001 1.688375L4.0813749999999995 0.6628125"
                        stroke="currentColor"
                        stroke-linecap="round"
                        stroke-width="1"
                    ></path><path
                        d="M13.311625 12.969875L11.944187500000002 12.4229375"
                        stroke="currentColor"
                        stroke-linecap="round"
                        stroke-width="1"
                    ></path><path
                        d="M13.311625 4.765L11.944187500000002 5.3119375"
                        stroke="currentColor"
                        stroke-linecap="round"
                        stroke-width="1"
                    ></path><path
                        d="M1.688375 12.969875L3.0558125 12.4229375"
                        stroke="currentColor"
                        stroke-linecap="round"
                        stroke-width="1"
                    ></path><path
                        d="M1.688375 4.765L3.0558125 5.3119375"
                        stroke="currentColor"
                        stroke-linecap="round"
                        stroke-width="1"
                    ></path><path
                        d="M7.5 13.995375000000001V9.551187500000001"
                        stroke="currentColor"
                        stroke-linecap="round"
                        stroke-width="1"
                    ></path></svg
                >
            </div>

            <div class="cursor-pointer">
                <svg
                    viewBox="-0.5 -0.5 16 16"
                    fill="none"
                    xmlns="http://www.w3.org/2000/svg"
                    height="16"
                    width="16"
                    ><path
                        d="M1.3464999999999998 8.867437500000001v3.008375c0 0.2265625 0.1836875 0.41025 0.41025 0.41025h3.6920625"
                        stroke="currentColor"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="1"
                    ></path><path
                        d="M11.6023125 8.867437500000001v3.008375c0 0.2265625 -0.183625 0.41025 -0.4101875 0.41025H8.18375"
                        stroke="currentColor"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="1"
                    ></path><path
                        d="M8.18375 2.7139375h3.008375c0.2265625 0 0.4101875 0.1836875 0.4101875 0.41025v3.008375"
                        stroke="currentColor"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="1"
                    ></path><path
                        d="M1.3464999999999998 6.1325625V3.1241875c0 -0.2265625 0.1836875 -0.41025 0.41025 -0.41025h3.6920625"
                        stroke="currentColor"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="1"
                    ></path><path
                        d="M8.18375 12.2860625v0.6836875c0 0.75525 -0.61225 1.3674374999999999 -1.3675000000000002 1.3674374999999999s-1.3674374999999999 -0.6121875 -1.3674374999999999 -1.3674374999999999v-0.6836875"
                        stroke="currentColor"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="1"
                    ></path><path
                        d="M1.3464999999999998 6.1325625h0.6837500000000001c0.7551874999999999 0 1.3674374999999999 0.6121875 1.3674374999999999 1.3674374999999999s-0.61225 1.3674374999999999 -1.3674374999999999 1.3674374999999999h-0.6837500000000001"
                        stroke="currentColor"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="1"
                    ></path><path
                        d="M11.6023125 6.1325625h0.6837500000000001c0.75525 0 1.3674374999999999 0.6121875 1.3674374999999999 1.3674374999999999s-0.6121875 1.3674374999999999 -1.3674374999999999 1.3674374999999999h-0.6837500000000001"
                        stroke="currentColor"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="1"
                    ></path><path
                        d="M8.18375 2.7139375V2.03025c0 -0.75525 -0.61225 -1.3674374999999999 -1.3675000000000002 -1.3674374999999999S5.4488125 1.275 5.4488125 2.03025v0.6836875"
                        stroke="currentColor"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="1"
                    ></path></svg
                >
            </div>

            <div class="cursor-pointer">

            </div>
        </div>
    </div>
</nav>
