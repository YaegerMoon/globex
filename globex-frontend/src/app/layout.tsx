import type { Metadata } from 'next';
import { Inter } from 'next/font/google';
import './globals.css';
import Providers from '@/components/Providers';

const inter = Inter({ subsets: ['latin'] });

export const metadata: Metadata = {
  title: 'Globex - Global Market Comparison',
  description: 'Compare global markets and track fear & greed index',
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body className={inter.className}>
        <Providers>
          <main className="min-h-screen bg-gray-50">
            <nav className="bg-white border-b border-gray-200 py-4 px-8 mb-8">
              <div className="max-w-7xl mx-auto flex justify-between items-center">
                <h1 className="text-2xl font-bold text-blue-600">Globex</h1>
                <div className="flex gap-6">
                  <a href="/" className="text-gray-600 hover:text-blue-600 font-medium">Dashboard</a>
                  <a href="#" className="text-gray-600 hover:text-blue-600 font-medium">About</a>
                </div>
              </div>
            </nav>
            <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
              {children}
            </div>
          </main>
        </Providers>
      </body>
    </html>
  );
}
