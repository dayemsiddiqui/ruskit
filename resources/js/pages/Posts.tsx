import React from 'react';
import { Head } from '@inertiajs/react';
import type { PostListProps } from '../types/generated';

export default function Posts({ posts }: PostListProps) {
    return (
        <>
            <Head title="Posts" />
            <div className="max-w-7xl mx-auto py-12 px-4 sm:px-6 lg:px-8">
                <div className="text-center">
                    <h1 className="text-4xl font-bold text-gray-900 sm:text-5xl">
                        Posts
                    </h1>
                    <div className="mt-8">
                        <div className="grid grid-cols-1 gap-8 sm:grid-cols-2 lg:grid-cols-3">
                            {posts.map((post) => (
                                <div key={post.id} className="bg-white shadow-md rounded-lg overflow-hidden">
                                    <div className="p-6">
                                        <h2 className="text-lg font-medium text-gray-900">{post.title}</h2>
                                        <p className="mt-2 text-sm text-gray-600">{post.content}</p>
                                    </div>
                                </div>
                            ))}
                        </div>
                    </div>
                </div>
            </div>
        </>
    );
}