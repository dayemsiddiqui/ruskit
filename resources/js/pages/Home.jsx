import React from 'react';

export default function Home() {
  return (
    <div className="p-4">
      <h1 className="text-2xl font-bold">Welcome to Rustkit</h1>
      <p className="mt-2">Your Rust + Laravel-inspired application is ready!</p>

      <h3>Features</h3>
      <ul>
        <li>Inertia</li>
        <li>Tailwind CSS</li>
        <li>React</li>
        <li>Rust</li>
      </ul>
    </div>
  )
}