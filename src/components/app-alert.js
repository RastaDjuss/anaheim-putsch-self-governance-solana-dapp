import { AlertCircle } from 'lucide-react';
import { Alert, AlertDescription, AlertTitle } from '../../src/components/ui/alert';
export function AppAlert({ action, children }) {
    return (<Alert variant="warning">
      <AlertCircle className="h-4 w-4"/>
      <AlertTitle>{children}</AlertTitle>
      <AlertDescription className="flex justify-end">{action}</AlertDescription>
    </Alert>);
}
