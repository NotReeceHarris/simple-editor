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
  let editor: monaco.editor.IStandaloneCodeEditor;
  let Monaco: typeof monaco;

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

  // Use onMount lifecycle function to set up the editor
  // @ts-ignore is used to bypass TypeScript errors
  onMount(async () => {

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
        width: window.innerWidth,
        height: window.innerHeight,
      });
    };

    // Add event listener for window resize
    window.addEventListener("resize", resize);

    // Initial resize and breadcrumbs update
    resize();

    onDestroy(() => {
      if (editor) {
        editor.dispose();
      }
      window.removeEventListener("resize", resize);
    });
  });
</script>

<div bind:this={editorElement} class="h-screen"></div>

<style>
  :global(.monaco-editor) {
    width: 100% !important;
    height: 100% !important;
  }
</style>
