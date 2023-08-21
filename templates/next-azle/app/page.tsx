"use client";
import { useState } from "react";
import { makeAzleActor } from "../service/actor";

export default function Home() {
  const [message, setMessage] = useState<string>("");
  const [isButtonDisabled, setIsButtonDisabled] = useState(false);
  const [desiredStateReached, setDesiredStateReached] = useState(false);

  const handleSubmit = async () => {
    try {
      const azle = await makeAzleActor();

      setIsButtonDisabled(true);
      const getRandomByte = await azle.randomBytes();
      console.log("getRandomByte ", getRandomByte);

      setMessage(getRandomByte);
      setIsButtonDisabled(false);
    } catch (error) {
      console.log(JSON.stringify(error));
    }
    setIsButtonDisabled(false);
  };
  return (
    <main>
      <div></div>
      <div>
        <div>
          {" "}
          <h1 className="flex items-center justify-center mt-10 italic mb-4 text-4xl font-extrabold leading-none tracking-tight text-gray-900 md:text-5xl lg:text-6xl dark:text-white ">
            NextJS + Azle Canister Template
          </h1>
        </div>
        <br />

        <br />
        <div>
          <button
            type="button"
            onClick={handleSubmit}
            disabled={isButtonDisabled}
            className="mx-auto flex items-center p-5 justify-center mt-10 text-white bg-gradient-to-br from-purple-600 to-blue-500 hover:bg-gradient-to-bl focus:ring-4 focus:outline-none focus:ring-blue-300 dark:focus:ring-blue-800 font-medium rounded-lg text-sm px-5"
          >
            {" "}
            Random Bytes
          </button>
        </div>
        <div>
          {" "}
          <h3 className="flex items-center justify-center mt-10  text-2xl ">
            {message}
          </h3>
        </div>
      </div>
    </main>
  );
}
