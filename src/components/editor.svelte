<script lang="ts">
  // Import necessary Svelte lifecycle function and types
  import { onMount, onDestroy } from "svelte";
  import type monaco from "monaco-editor";

  // Import Monaco Editor workers
  import editorWorker from "monaco-editor/esm/vs/editor/editor.worker?worker";
  import jsonWorker from "monaco-editor/esm/vs/language/json/json.worker?worker";
  import cssWorker from "monaco-editor/esm/vs/language/css/css.worker?worker";
  import htmlWorker from "monaco-editor/esm/vs/language/html/html.worker?worker";
  import tsWorker from "monaco-editor/esm/vs/language/typescript/ts.worker?worker";

  // Declare variables for DOM elements and Monaco types
  let editorElement: HTMLDivElement;
  let breadcrumbsElement: HTMLDivElement;
  let editor: monaco.editor.IStandaloneCodeEditor;
  let Monaco: typeof monaco;
  let Sidebar: HTMLElement;

  // Use onMount lifecycle function to set up the editor
  // @ts-ignore is used to bypass TypeScript errors
  onMount(async () => {
    // Get the sidebar element
    // @ts-ignore
    Sidebar = document.getElementById("sidebar");

    // Set up Monaco environment with appropriate workers
    // @ts-ignore
    self.MonacoEnvironment = {
      getWorker: function (_moduleId: any, label: string) {
        // Return appropriate worker based on language
        if (label === "json") {
          return new jsonWorker();
        }
        if (label === "css" || label === "scss" || label === "less") {
          return new cssWorker();
        }
        if (label === "html" || label === "handlebars" || label === "razor") {
          return new htmlWorker();
        }
        if (label === "typescript" || label === "javascript") {
          return new tsWorker();
        }
        return new editorWorker();
      },
    };

    // Dynamically import Monaco editor
    Monaco = await import("monaco-editor");

    // Define a custom theme for the editor
    Monaco.editor.defineTheme("default", {
      base: "vs-dark",
      inherit: true,
      rules: [
        { token: "identifier", foreground: "9CDCFE" },
        { token: "identifier.function", foreground: "DCDCAA" },
        { token: "type", foreground: "1AAFB0" },
      ],
      colors: {
        "editor.background": "#27282a",
        "scrollbar.shadow": "#27282a",
        "scrollbarSlider.background": "#353638b3",
      },
    });

    // Create the Monaco editor instance
    editor = Monaco.editor.create(editorElement, {
      value: [
        "function helloWorld() {",
        "  console.log('Hello world!');",
        "}",
        "",
        "class Example {",
        "  constructor() {",
        "    this.greeting = 'Hello';",
        "  }",
        "",
        "  sayHello() {",
        "    console.log(this.greeting);",
        "  }",
        "}",
      ].join("\n"),
      automaticLayout: false,
      language: "javascript",
      theme: "default",
    });

    // Function to resize the editor
    const resize = () => {
      editor.layout({
        width: window.innerWidth - Math.max(Sidebar ? Sidebar.clientWidth : 0, 10),
        height: window.innerHeight - 144,
      });
    };

    // Add event listener for window resize
    window.addEventListener("resize", resize);

    // Create a ResizeObserver to watch for changes in the sidebar size
    const resizeObserver = new ResizeObserver(resize);
    resizeObserver.observe(Sidebar);

    // Initial resize and breadcrumbs update
    resize();

    onDestroy(() => {
      if (editor) {
        editor.dispose();
      }
      window.removeEventListener("resize", resize);
      if (resizeObserver) {
        resizeObserver.disconnect();
      }
    });
  });
</script>

<div class="h-full flex">
  <div class="grow flex flex-col">
    <div
      class="border-b border-zinc-700 h-9 min-h-9 relative flex place-items-center pl-3"
      bind:this={breadcrumbsElement}
    ></div>
    <div bind:this={editorElement} class="h-screen"></div>
  </div>
</div>

<style>
  :global(.monaco-editor) {
    width: 100% !important;
  }
</style>
