import React from 'react';
import { Link } from '@inertiajs/react';

interface Post {
  id: number;
  title: string;
  content: string;
  slug: string;
  created_at: string;
  updated_at: string;
}

interface Props {
  post: Post;
}

export default function Show({ post }: Props) {
  return (
    <div className="py-12">
      <div className="max-w-7xl mx-auto sm:px-6 lg:px-8">
        <div className="bg-white overflow-hidden shadow-sm sm:rounded-lg">
          <div className="p-6 bg-white border-b border-gray-200">
            <div className="flex justify-between items-center mb-6">
              <h1 className="text-2xl font-semibold text-gray-900">{post.title}</h1>
              <div className="space-x-4">
                <Link
                  href={`/posts/${post.id}/edit`}
                  className="px-4 py-2 bg-indigo-600 text-white rounded-md hover:bg-indigo-700"
                >
                  Edit
                </Link>
                <Link
                  href="/posts"
                  className="px-4 py-2 bg-gray-600 text-white rounded-md hover:bg-gray-700"
                >
                  Back to Posts
                </Link>
              </div>
            </div>

            <div className="prose max-w-none">
              <div className="mb-4">
                <h2 className="text-lg font-semibold text-gray-700">Content</h2>
                <div className="mt-2 text-gray-600">{post.content}</div>
              </div>

              <div className="mt-6 text-sm text-gray-500">
                <p>Slug: {post.slug}</p>
                <p>Created: {new Date(post.created_at).toLocaleString()}</p>
                <p>Last Updated: {new Date(post.updated_at).toLocaleString()}</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
} 