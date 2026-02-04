import { useState } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Textarea } from '@/components/ui/textarea';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import * as Icons from '@hugeicons/react';

interface ApiPlaygroundProps {
  // TODO: Define props for the generated API client instance
}

export const ApiPlayground = ({ /* props */ }: ApiPlaygroundProps) => {
  const [selectedEndpoint, setSelectedEndpoint] = useState('');
  const [requestBody, setRequestBody] = useState('');
  const [apiResponse, setApiResponse] = useState('');

  // Placeholder for available API endpoints
  const endpoints = [
    { value: '/users', label: 'GET /users' },
    { value: '/users/:id', label: 'GET /users/:id' },
    { value: '/auth/login', label: 'POST /auth/login' },
  ];

  const handleExecuteRequest = () => {
    if (selectedEndpoint) {
      setApiResponse('Executing API call...');
      // TODO: Implement actual API call using the generated client
      setTimeout(() => {
        setApiResponse(JSON.stringify({
          endpoint: selectedEndpoint,
          requestBody: requestBody ? JSON.parse(requestBody) : null,
          mockResponse: 'Success! (Mock response)'
        }, null, 2));
      }, 1000);
    } else {
      setApiResponse('Please select an API endpoint.');
    }
  };

  return (
    <div className="space-y-6">
      <h2 className="text-xl font-semibold">Developer Tools - API Playground</h2>

      <Card>
        <CardHeader>
          <CardTitle>Test Hey API Endpoints</CardTitle>
        </CardHeader>
        <CardContent className="grid gap-4">
          <div className="grid gap-2">
            <Label htmlFor="endpoint">Select Endpoint</Label>
            <Select value={selectedEndpoint} onValueChange={(value) => setSelectedEndpoint(value || '')}>
              <SelectTrigger id="endpoint">
                <SelectValue placeholder="Choose an API endpoint" />
              </SelectTrigger>
              <SelectContent>
                {endpoints.map(ep => (
                  <SelectItem key={ep.value} value={ep.value}>{ep.label}</SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>

          <div className="grid gap-2">
            <Label htmlFor="requestBody">Request Body (JSON)</Label>
            <Textarea
              id="requestBody"
              rows={5}
              value={requestBody}
              onChange={(e) => setRequestBody(e.target.value)}
              placeholder="Enter JSON request body if applicable"
            />
          </div>

          <Button onClick={handleExecuteRequest} className="w-fit">
            <Icons.Code className="h-4 w-4 mr-2" />Execute Request
          </Button>

          <div className="grid gap-2">
            <Label htmlFor="apiResponse">API Response</Label>
            <Textarea
              id="apiResponse"
              rows={10}
              readOnly
              value={apiResponse}
              placeholder="API response will appear here."
              className="font-mono text-sm"
            />
          </div>
        </CardContent>
      </Card>
    </div>
  );
};
