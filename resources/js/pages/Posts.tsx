import React from 'react';
import { Head } from '@inertiajs/react';
import type { PostsProps } from '../types/generated';

interface Props extends PostsProps {}

export default function Posts({ title }: Props) {
    return (
        <>
            <Head title={title} />
            <div className="max-w-7xl mx-auto py-12 px-4 sm:px-6 lg:px-8">
                <div className="text-center">
                    <h1 className="text-4xl font-bold text-gray-900 sm:text-5xl">
                        {title}
                    </h1>
                    {/* Add your page content here */}
                </div>
            </div>
        </>
    );
}