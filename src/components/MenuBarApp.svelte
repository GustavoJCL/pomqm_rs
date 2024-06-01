<script lang="ts">
  import * as Menubar from "$lib/components/ui/menubar/index.js";
  import "../app.css";
  import Printer from "@tabler/icons-svelte/IconPrinter.svelte";
  import {
    IconCaretRight,
    IconColumnInsertLeft,
    IconColumnRemove,
    IconFileSpreadsheet,
    IconRowInsertTop,
    IconRowRemove,
  } from "@tabler/icons-svelte";
  import ThemeSwitcher from "./ThemeSwitcher.svelte";
  import { Content } from "$lib/components/ui/dropdown-menu";

  let bookmarks = false;
  let fullUrls = true;

  const profileRadioValue = "benoit";
</script>

<Menubar.Root>
  <Menubar.Menu>
    <Menubar.Trigger>File</Menubar.Trigger>
    <Menubar.Content class="flex flex-col">
      <Menubar.Item class="text-xs">
        New <Menubar.Shortcut>⌘T</Menubar.Shortcut>
      </Menubar.Item>
      <Menubar.Item class="text-xs">
        Open <Menubar.Shortcut>⌘N</Menubar.Shortcut>
      </Menubar.Item>
      <Menubar.Item class="text-xs">New</Menubar.Item>
      <Menubar.Separator />
      <Menubar.Item class="text-xs">Save</Menubar.Item>
      <Menubar.Item class="text-xs">Save as</Menubar.Item>
      <Menubar.Item class="text-xs">
        <IconFileSpreadsheet size={20} />
        Save as a Excel File</Menubar.Item
      >
      <Menubar.Separator />
      <Menubar.Item class="text-xs">
        <Printer size={20} />
        Print
        <Menubar.Shortcut>⌘P</Menubar.Shortcut>
      </Menubar.Item>
      <Menubar.Separator />
      <Menubar.Item class="text-xs">Solve</Menubar.Item>
      <Menubar.Separator />
      <Menubar.Item class="text-xs">Exit</Menubar.Item>
    </Menubar.Content>
  </Menubar.Menu>
  <Menubar.Menu>
    <Menubar.Trigger>Edit</Menubar.Trigger>
    <Menubar.Content>
      <Menubar.Item class="text-xs">
        <IconRowInsertTop size={20} />
        Insert Row(s)
      </Menubar.Item>
      <Menubar.Item class="text-xs">
        <IconColumnInsertLeft size={20} />
        Insert Column(s)</Menubar.Item
      >
      <Menubar.Separator />
      <Menubar.Item class="text-xs">
        <IconRowRemove size={20} />
        Delete Row(s)
      </Menubar.Item>
      <Menubar.Item class="text-xs">
        <IconColumnRemove size={20} />
        Delete Column(s)</Menubar.Item
      >
      <Menubar.Separator />
      <Menubar.Item class="text-xs">
        Copy <Menubar.Shortcut>⌘c</Menubar.Shortcut>
      </Menubar.Item>
      <Menubar.Item class="text-xs">
        Copy Table with Headers<Menubar.Shortcut>⌘C</Menubar.Shortcut>
      </Menubar.Item>
      <Menubar.Item class="text-xs">
        Paste <Menubar.Shortcut>⌘v</Menubar.Shortcut>
      </Menubar.Item>
      <Menubar.Item class="text-xs">Copy down</Menubar.Item>
    </Menubar.Content>
  </Menubar.Menu>
  <Menubar.Menu>
    <Menubar.Trigger>View</Menubar.Trigger>
    <Menubar.Content>
      <Menubar.CheckboxItem bind:checked={bookmarks}
        >Always Show Bookmarks Bar</Menubar.CheckboxItem
      >
      <Menubar.CheckboxItem bind:checked={fullUrls}>
        Always Show Full URLs
      </Menubar.CheckboxItem>
      <Menubar.Separator />
      <Menubar.Item inset>
        Reload <Menubar.Shortcut>⌘R</Menubar.Shortcut>
      </Menubar.Item>
      <Menubar.Item inset>
        Force Reload <Menubar.Shortcut>⇧⌘R</Menubar.Shortcut>
      </Menubar.Item>
      <Menubar.Separator />
      <Menubar.Item inset>Toggle Fullscreen</Menubar.Item>
      <Menubar.Separator />
      <Menubar.Item inset>Hide Sidebar</Menubar.Item>
    </Menubar.Content>
  </Menubar.Menu>
  <Menubar.Menu>
    <Menubar.Trigger>Taylor</Menubar.Trigger>
    <Menubar.Content>
      <Menubar.RadioGroup value={profileRadioValue}>
        <Menubar.RadioItem value="andy">Andy</Menubar.RadioItem>
        <Menubar.RadioItem value="benoit">Benoit</Menubar.RadioItem>
        <Menubar.RadioItem value="Luis">Luis</Menubar.RadioItem>
      </Menubar.RadioGroup>
      <Menubar.Separator />
      <Menubar.Item inset>Edit...</Menubar.Item>
      <Menubar.Separator />
      <Menubar.Item inset>Add Profile...</Menubar.Item>
    </Menubar.Content>
  </Menubar.Menu>
  <Menubar.Menu>
    <Menubar.Trigger>Module</Menubar.Trigger>
    <Menubar.Content>
      <Menubar.Item>Assigment</Menubar.Item>

      <Menubar.Item>Breakeven/Cost-Volume Analysis</Menubar.Item>
      <Menubar.Item>Decision Analysis</Menubar.Item>
      <Menubar.Item>Forecasting</Menubar.Item>
      <Menubar.Item>Game Theory</Menubar.Item>
      <Menubar.Item>Goal Programming</Menubar.Item>
      <Menubar.Item>Integer-Mixed Integer Programming</Menubar.Item>
      <Menubar.Item>Invetory</Menubar.Item>
      <Menubar.Item>Linear Programming</Menubar.Item>
      <Menubar.Item>Markov Analysis</Menubar.Item>
    </Menubar.Content>
  </Menubar.Menu>
  <Menubar.Menu>
    <Menubar.Trigger>Format</Menubar.Trigger>
  </Menubar.Menu>
  <Menubar.Menu>
    <Menubar.Trigger>Tools</Menubar.Trigger>
  </Menubar.Menu>
  <Menubar.Menu>
    <Menubar.Trigger><IconCaretRight /> Solution</Menubar.Trigger>
    <Menubar.Content>
      <Menubar.RadioGroup value={profileRadioValue}>
        <Menubar.RadioItem value="andy">Andy</Menubar.RadioItem>
        <Menubar.RadioItem value="benoit">Benoit</Menubar.RadioItem>
        <Menubar.RadioItem value="Luis">Luis</Menubar.RadioItem>
      </Menubar.RadioGroup>
      <Menubar.Separator />
      <Menubar.Item inset>Edit...</Menubar.Item>
      <Menubar.Separator />
      <Menubar.Item inset>Add Profile...</Menubar.Item>
    </Menubar.Content>
  </Menubar.Menu>
  <Menubar.Menu>
    <Menubar.Trigger>Help</Menubar.Trigger>
  </Menubar.Menu>
  <div class="w-full" />
  <ThemeSwitcher />
</Menubar.Root>
