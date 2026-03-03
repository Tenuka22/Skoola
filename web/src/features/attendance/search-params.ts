import { parseAsInteger, parseAsString, useQueryState } from 'nuqs'
import { addDays, format } from 'date-fns'

export const useAttendanceSearchParams = () => {
  const [date, setDate] = useQueryState(
    'date',
    parseAsString.withDefault(format(new Date(), 'yyyy-MM-dd')),
  )
  const [classId, setClassId] = useQueryState('class_id', parseAsString)
  const [staffType, setStaffType] = useQueryState('staff_type', parseAsString)

  const [fromDate, setFromDate] = useQueryState(
    'from_date',
    parseAsString.withDefault(format(addDays(new Date(), -30), 'yyyy-MM-dd')),
  )
  const [toDate, setToDate] = useQueryState(
    'to_date',
    parseAsString.withDefault(format(new Date(), 'yyyy-MM-dd')),
  )

  const [threshold, setThreshold] = useQueryState(
    'threshold',
    parseAsInteger.withDefault(80),
  )

  return {
    date,
    setDate,
    classId,
    setClassId,
    staffType,
    setStaffType,
    fromDate,
    setFromDate,
    toDate,
    setToDate,
    threshold,
    setThreshold,
  }
}
