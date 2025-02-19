import React from 'react';
import { Head } from '@inertiajs/react';
import { AboutPageProps } from '../types/generated';

export default function About({ title, description, tech_stack }: AboutPageProps) {
    return (
        <>
            <Head title="About Us" />
            <div className="min-h-screen bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
                <div className="max-w-4xl mx-auto">
                    <div className="bg-white rounded-2xl shadow-xl overflow-hidden">
                        <div className="px-6 py-8 sm:p-10">
                            <div className="border-b border-gray-200 pb-8">
                                <h1 className="text-4xl font-bold text-gray-900 tracking-tight">{title}</h1>
                                <p className="mt-4 text-lg text-gray-600">
                                    {description}
                                </p>
                            </div>
                            
                            <div className="mt-8 grid gap-8 grid-cols-1 md:grid-cols-2">
                                <div className="bg-gray-50 rounded-xl p-6">
                                    <h2 className="text-xl font-semibold text-gray-900">Our Tech Stack</h2>
                                    <ul className="mt-4 space-y-3 text-gray-600">
                                        {tech_stack.map((tech, index) => (
                                            <li key={`tech-${index}`} className="flex items-center">
                                                <span className="bg-green-100 text-green-800 px-3 py-1 rounded-full text-sm font-medium">
                                                    {tech}
                                                </span>
                                            </li>
                                        ))}
                                    </ul>
                                </div>

                                <div className="bg-gray-50 rounded-xl p-6">
                                    <h2 className="text-xl font-semibold text-gray-900">Why Choose Us?</h2>
                                    <ul className="mt-4 space-y-3 text-gray-600">
                                        <li className="flex items-start">
                                            <svg className="h-6 w-6 text-green-500 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
                                            </svg>
                                            High Performance
                                        </li>
                                        <li className="flex items-start">
                                            <svg className="h-6 w-6 text-green-500 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
                                            </svg>
                                            Type Safety
                                        </li>
                                        <li className="flex items-start">
                                            <svg className="h-6 w-6 text-green-500 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
                                            </svg>
                                            Modern UI/UX
                                        </li>
                                    </ul>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </>
    );
} 