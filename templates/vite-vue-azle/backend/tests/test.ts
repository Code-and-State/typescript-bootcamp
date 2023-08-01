import { runTests } from 'azle/test';
import { get_tests } from './tests';
import { backend}  from './actor';

runTests(get_tests(backend));