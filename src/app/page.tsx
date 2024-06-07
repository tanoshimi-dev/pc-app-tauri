"use client"

import Image from "next/image";
import React from "react";

import { open } from '@tauri-apps/api/dialog';
import { appDir } from '@tauri-apps/api/path';
import { ask } from '@tauri-apps/api/dialog';
import { invoke } from "@tauri-apps/api/tauri";

interface Message {
  field_str: string;
  field_u32: number;
}

export default function Home() {

  const [greetMsg, setGreetMsg] = React.useState<string>("");
  const [name, setName] = React.useState("");
 
  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }

  function executeCommands() {
    //invoke('simple_command')

    for (let arg of [1, 2]) {
      invoke('command_with_error', { arg }).then(message => {
        console.log('command_with_error', message)
      }).catch(message => {
        console.error('command_with_error', message)
      })
    }    

    
    invoke('command_with_object', { message: { field_str: 'some message', field_u32: 12 }})
      .then((message: unknown) => {
      console.log('command_with_object', message)

      if (message && (message as Message).field_str !== undefined) {
        setGreetMsg((message as Message).field_str)

      }

    })

  }

  
  function execute() {

    // console.log('execute', selectedInputDirectory[0], selectedOutputDirectory[0])

    invoke('execute',  
      { directories: 
        { input_directory: selectedInputDirectory ? selectedInputDirectory[0] : null, 
          output_directory: selectedOutputDirectory ? selectedOutputDirectory[0] : null }}).then(resultMessage => {

      console.log('execute 1！', resultMessage)
    }).catch(directories => {
      console.error('execute 2💀', directories)
    })    

  }

  const [selectedInputDirectory, setSelectedInputDirectory] = React.useState<String | null>(null);
  const [selectedOutputDirectory, setSelectedOutputDirectory] = React.useState<String | null>(null);

  // const fileInputRef = React.useRef<HTMLInputElement>(null);

  React.useEffect(() => {
    greet();
    // if (fileInputRef.current) {
    //   fileInputRef.current.setAttribute('webkitdirectory', '');
    // }
  }, []);


const selectInputDir =  async function () {
  const directory = await open({
    multiple: true,
    directory: true,
  });

  console.log('directory')  
  console.log(directory)
  setSelectedInputDirectory(directory as String | null);

  if (Array.isArray(directory)) {
    // user selected multiple files
  } else if (directory === null) {
    // user cancelled the selection
  } else {
    // user selected a single file
    //console.log(directory)
  }

}

const selectOutputDir =  async function () {
  const directory = await open({
    multiple: true,
    directory: true,
  });

  console.log('directory')  
  console.log(directory)
  setSelectedOutputDirectory(directory as String | null);

  if (Array.isArray(directory)) {
    // user selected multiple files
  } else if (directory === null) {
    // user cancelled the selection
  } else {
    // user selected a single file
    //console.log(directory)
  }

}

  const openDialog = async () => {
    const yes = await ask('Are you sure?', 'Tauri');
    const yes2 = await ask('This action cannot be reverted. Are you sure?', { title: 'Tauri', type: 'warning' });
  }

  return (
    <main className="flex min-h-screen flex-col items-center justify-between py-12">
      
      {/* <div className="z-10 w-full max-w-5xl items-center justify-between font-mono text-sm lg:flex">
        <p className="fixed left-0 top-0 flex w-full justify-center border-b border-gray-300 bg-gradient-to-b from-zinc-200 pb-6 pt-8 backdrop-blur-2xl dark:border-neutral-800 dark:bg-zinc-800/30 dark:from-inherit lg:static lg:w-auto  lg:rounded-xl lg:border lg:bg-gray-200 lg:p-4 lg:dark:bg-zinc-800/30">
          Get started by editing&nbsp;
          <code className="font-mono font-bold">src/app/page.tsx</code>
        </p>
        <div className="fixed bottom-0 left-0 flex h-48 w-full items-end justify-center bg-gradient-to-t from-white via-white dark:from-black dark:via-black lg:static lg:size-auto lg:bg-none">
          <a
            className="pointer-events-none flex place-items-center gap-2 p-8 lg:pointer-events-auto lg:p-0"
            href="https://vercel.com?utm_source=create-next-app&utm_medium=appdir-template&utm_campaign=create-next-app"
            target="_blank"
            rel="noopener noreferrer"
          >
            By{" "}
            <Image
              src="/vercel.svg"
              alt="Vercel Logo"
              className="dark:invert"
              width={100}
              height={24}
              priority
            />
          </a>
        </div>
      </div> */}
      
      {/* <div className="relative z-[-1] flex place-items-center before:absolute before:h-[300px] before:w-full before:-translate-x-1/2 before:rounded-full before:bg-gradient-radial before:from-white before:to-transparent before:blur-2xl before:content-[''] after:absolute after:-z-20 after:h-[180px] after:w-full after:translate-x-1/3 after:bg-gradient-conic after:from-sky-200 after:via-blue-200 after:blur-2xl after:content-[''] before:dark:bg-gradient-to-br before:dark:from-transparent before:dark:to-blue-700 before:dark:opacity-10 after:dark:from-sky-900 after:dark:via-[#0141ff] after:dark:opacity-40 sm:before:w-[480px] sm:after:w-[240px] before:lg:h-[360px]">
        <Image
          className="relative dark:drop-shadow-[0_0_0.3rem_#ffffff70] dark:invert"
          src="/next.svg"
          alt="Next.js Logo"
          width={180}
          height={37}
          priority
        />
      </div> */}
      
      <div>

        <p>Click on the Tauri, Vite, and React logos to learn more.</p>

        <div className="row">
          <form
            onSubmit={(e) => {
              e.preventDefault();
              greet();
            }}
          >
            <input
              id="greet-input"
              onChange={(e) => setName(e.currentTarget.value)}
              placeholder="Enter a name..."
            />
            <button type="submit">Greet</button>
          </form>

        </div>
        <p>{greetMsg}</p>

        <div>Hello Tauri</div>
        <button onClick={executeCommands}>Click to execute command</button>

        <hr className="my-2"/>
        <div className="row py-2">
          <button className="p-2 rounded bg-indigo-200" onClick={selectInputDir}>Select Input Directory</button>
          <div>
          Input Directory:
          </div>
          <div>
            {selectedInputDirectory}
          </div>
        </div>

        <div className="row py-2">
          <button className="p-2 rounded bg-rose-200" onClick={selectOutputDir}>Select Output Directory</button>
          <div>
          Output Directory:
          </div>
          <div>
            {selectedOutputDirectory}
          </div>
        </div>

        <button className="p-2 rounded bg-green-200" onClick={execute}>Click to execute</button>


        {/* <div className="row">
          <button onClick={openDialog}>openDialog</button>
        </div> */}

      </div>


    </main>
  );
}
