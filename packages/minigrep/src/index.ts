import * as path from 'path';
import * as minimist from 'minimist';
import * as binding from '../binding';

(function main() {
    let start = Date.now();
    const args = minimist(process.argv.slice(2));
    const { query, filename } = args as binding.Params;
    
    const ignoreCase = process.env.CASE_INSENSITIVE === '1';
    
    binding.search({
        query,
        filename: path.resolve(__dirname, filename),
        ignoreCase,
    });

    console.log(`Elapsed time: ${Date.now() - start} ms`);
})();