import { useState } from 'react'
import { HugeiconsIcon } from '@hugeicons/react'
import { BookOpen01Icon, Book02Icon } from '@hugeicons/core-free-icons'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Button } from '@/components/ui/button'

type CirculationScannerProps = object
// TODO: Define props for issue/return functions

export const CirculationScanner = (_props: CirculationScannerProps) => {
  const [bookIdentifier, setBookIdentifier] = useState('')
  const [userIdentifier, setUserIdentifier] = useState('')
  const [feedback, setFeedback] = useState('')

  const handleIssueBook = () => {
    if (bookIdentifier && userIdentifier) {
      setFeedback(
        `Attempting to ISSUE book '${bookIdentifier}' to user '${userIdentifier}'.`,
      )
      // TODO: Implement actual issue logic via API
      setBookIdentifier('')
      setUserIdentifier('')
    } else {
      setFeedback('Please enter both Book and User Identifiers to issue.')
    }
  }

  const handleReturnBook = () => {
    if (bookIdentifier) {
      setFeedback(`Attempting to RETURN book '${bookIdentifier}'.`)
      // TODO: Implement actual return logic via API
      setBookIdentifier('')
    } else {
      setFeedback('Please enter a Book Identifier to return.')
    }
  }

  return (
    <div className="space-y-6">
      <h2 className="text-xl font-semibold">Library Circulation Scanner</h2>

      <Card>
        <CardHeader>
          <CardTitle>Issue / Return Books</CardTitle>
        </CardHeader>
        <CardContent className="grid gap-4">
          <div className="grid gap-2">
            <Label htmlFor="bookIdentifier">Book ISBN / Barcode</Label>
            <Input
              id="bookIdentifier"
              value={bookIdentifier}
              onChange={(e) => setBookIdentifier(e.target.value)}
              placeholder="Scan or enter Book ISBN/Barcode"
            />
          </div>
          <div className="grid gap-2">
            <Label htmlFor="userIdentifier">Student / Staff ID</Label>
            <Input
              id="userIdentifier"
              value={userIdentifier}
              onChange={(e) => setUserIdentifier(e.target.value)}
              placeholder="Scan or enter Student/Staff ID (for issuing)"
            />
          </div>

          <div className="flex gap-4 justify-between">
            <Button onClick={handleIssueBook} className="flex-grow">
              <HugeiconsIcon icon={BookOpen01Icon} className="h-5 w-5 mr-2" />
              Issue Book
            </Button>
            <Button
              onClick={handleReturnBook}
              className="flex-grow"
              variant="secondary"
            >
              <HugeiconsIcon icon={Book02Icon} className="h-5 w-5 mr-2" />
              Return Book
            </Button>
          </div>

          {feedback && (
            <p className="text-sm text-muted-foreground mt-4">{feedback}</p>
          )}
        </CardContent>
      </Card>
    </div>
  )
}
