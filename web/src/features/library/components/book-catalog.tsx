import { useState } from 'react'
import { HugeiconsIcon } from '@hugeicons/react'
import { Search01Icon, PlusSignIcon, Book01Icon } from '@hugeicons/core-free-icons'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Button } from '@/components/ui/button'
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table'

type BookCatalogProps = object
// TODO: Define props for book data, search function, add book function

interface Book {
  isbn: string
  title: string
  author: string
  availableCopies: number
  totalCopies: number
}

export const BookCatalog = (_props: BookCatalogProps) => {
  const [searchISBN, setSearchISBN] = useState('')
  const [books, setBooks] = useState<Array<Book>>([
    {
      isbn: '978-0321765723',
      title: 'The Lord of the Rings',
      author: 'J.R.R. Tolkien',
      availableCopies: 5,
      totalCopies: 10,
    },
    {
      isbn: '978-0743273565',
      title: 'The Great Gatsby',
      author: 'F. Scott Fitzgerald',
      availableCopies: 3,
      totalCopies: 5,
    },
  ])

  const handleSearch = () => {
    if (searchISBN) {
      alert(`Searching for ISBN: ${searchISBN}`)
      // TODO: Implement actual ISBN search via API
      // For now, filter mock data
      const foundBook = books.find((book) => book.isbn === searchISBN)
      if (foundBook) {
        setBooks([foundBook]) // Display only the found book
      } else {
        setBooks([]) // No book found
      }
    } else {
      setBooks([
        {
          isbn: '978-0321765723',
          title: 'The Lord of the Rings',
          author: 'J.R.R. Tolkien',
          availableCopies: 5,
          totalCopies: 10,
        },
        {
          isbn: '978-0743273565',
          title: 'The Great Gatsby',
          author: 'F. Scott Fitzgerald',
          availableCopies: 3,
          totalCopies: 5,
        },
      ]) // Reset to all mock books
    }
  }

  return (
    <div className="space-y-6">
      <h2 className="text-xl font-semibold">Library Book Catalog</h2>

      <Card>
        <CardHeader>
          <CardTitle>Search & Add Books</CardTitle>
        </CardHeader>
        <CardContent className="grid gap-4">
          <div className="flex gap-4 items-end">
            <div className="grid gap-2 flex-grow">
              <Label htmlFor="isbnSearch">Search by ISBN</Label>
              <Input
                id="isbnSearch"
                value={searchISBN}
                onChange={(e) => setSearchISBN(e.target.value)}
                placeholder="Enter ISBN (e.g., 978-0321765723)"
              />
            </div>
            <Button onClick={handleSearch}>
              <HugeiconsIcon icon={Search01Icon} className="h-4 w-4 mr-2" />
              Search
            </Button>
          </div>
          <Button variant="outline" className="w-fit">
            <HugeiconsIcon icon={PlusSignIcon} className="h-4 w-4 mr-2" />
            Add New Book
          </Button>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>Book List</CardTitle>
        </CardHeader>
        <CardContent>
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>ISBN</TableHead>
                <TableHead>Title</TableHead>
                <TableHead>Author</TableHead>
                <TableHead className="text-right">Available</TableHead>
                <TableHead className="text-right">Total</TableHead>
                <TableHead className="text-right">Actions</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {books.map((book) => (
                <TableRow key={book.isbn}>
                  <TableCell>{book.isbn}</TableCell>
                  <TableCell className="font-medium">{book.title}</TableCell>
                  <TableCell>{book.author}</TableCell>
                  <TableCell className="text-right">
                    {book.availableCopies}
                  </TableCell>
                  <TableCell className="text-right">
                    {book.totalCopies}
                  </TableCell>
                  <TableCell className="text-right">
                    <Button variant="ghost" size="sm">
                      <HugeiconsIcon icon={Book01Icon} className="h-4 w-4" /> Edit
                    </Button>
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </CardContent>
      </Card>
    </div>
  )
}
